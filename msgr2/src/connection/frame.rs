use std::io::Read;

use crate::{
    connection::encryption::FrameEncryption,
    frame::{FrameFormat, Preamble, REV1_SECURE_INLINE_SIZE},
    key::AES_GCM_SIG_SIZE,
};

fn start_rx_bytes(format: FrameFormat) -> usize {
    match format {
        FrameFormat::Rev0Crc => crate::frame::Preamble::SERIALIZED_SIZE,
        FrameFormat::Rev1Crc => crate::frame::Preamble::SERIALIZED_SIZE,
        FrameFormat::Rev0Secure => todo!(),
        FrameFormat::Rev1Secure => {
            Preamble::SERIALIZED_SIZE + REV1_SECURE_INLINE_SIZE + AES_GCM_SIG_SIZE
        }
    }
}

fn rest_blocks(preamble: &Preamble) -> Vec<usize> {
    match preamble.format {
        FrameFormat::Rev0Crc | FrameFormat::Rev1Crc => {
            preamble.data_and_epilogue_segments().collect()
        }
        FrameFormat::Rev0Secure => todo!(),
        FrameFormat::Rev1Secure => preamble
            .data_and_epilogue_segments()
            .map(|v| v + AES_GCM_SIG_SIZE)
            .collect(),
    }
}

#[derive(Debug)]
pub enum TxError {
    EncryptionFailed,
    Io(std::io::Error),
}

impl From<std::io::Error> for TxError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug)]
pub enum RxError {
    DecryptionFailed,
    DecodePreamble(String),
    PreambleTruncated,
    FrameDataTruncated,
    Io(std::io::Error),
}

impl From<std::io::Error> for RxError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug)]
pub struct Unstarted<'enc> {
    pub(crate) encryption: &'enc mut FrameEncryption,
}

#[derive(Debug)]
pub struct ReadPreamble<'enc> {
    pub(crate) encryption: &'enc mut FrameEncryption,
    pub(crate) preamble: Preamble,
    // Raw preamble data. Necessary for computing AuthSignature.
    pub(crate) preamble_data: [u8; Preamble::SERIALIZED_SIZE],
    /// The index up to which data has been decrypted.
    pub(crate) decrypted_len: usize,
}

#[derive(Debug)]
pub struct Completed {
    pub(crate) preamble_data: [u8; Preamble::SERIALIZED_SIZE],
    pub(crate) preamble: Preamble,
}

#[derive(Debug)]
pub struct RxFrame<'buf, T> {
    state: T,
    format: FrameFormat,
    /// The decrypted frame data, but still including
    /// all padding and an optional epilogue.
    frame_data: &'buf mut Vec<u8>,
}

impl<'buf, 'enc> RxFrame<'buf, Unstarted<'enc>> {
    fn handle_pre_data(self) -> Result<RxFrame<'buf, ReadPreamble<'enc>>, RxError> {
        let Self {
            state: Unstarted { encryption },
            format,
            frame_data,
        } = self;

        // Decrypt incoming data frame
        let tag_len = encryption
            .decrypt(frame_data)
            .map_err(|_| RxError::DecryptionFailed)?;

        // Truncate tag
        let new_len = frame_data.len().checked_sub(tag_len).unwrap();
        frame_data.truncate(new_len);

        // Get and parse preamble
        let preamble_data = *frame_data.first_chunk().expect("self.preamble_len() >= 32");
        let preamble = Preamble::parse(&preamble_data, format).map_err(RxError::DecodePreamble)?;

        match format {
            crate::frame::FrameFormat::Rev0Crc => frame_data.truncate(0),
            crate::frame::FrameFormat::Rev1Crc => frame_data.truncate(0),
            crate::frame::FrameFormat::Rev0Secure => todo!(),
            crate::frame::FrameFormat::Rev1Secure => {
                frame_data.copy_within(Preamble::SERIALIZED_SIZE.., 0);
                let non_trailer_data = preamble.segments()[0].len();
                frame_data.truncate(non_trailer_data);
            }
        }

        Ok(RxFrame {
            state: ReadPreamble {
                preamble_data,
                encryption: encryption,
                preamble,
                decrypted_len: frame_data.len(),
            },
            format,
            frame_data,
        })
    }

    pub fn new(
        format: FrameFormat,
        encryption: &'enc mut FrameEncryption,
        frame_buffer: &'buf mut Vec<u8>,
    ) -> Self {
        frame_buffer.clear();
        Self {
            state: Unstarted { encryption },
            frame_data: frame_buffer,
            format,
        }
    }

    pub fn read_preamble(
        mut self,
        read: impl std::io::Read,
    ) -> Result<RxFrame<'buf, ReadPreamble<'enc>>, RxError> {
        // Read pre data
        let mut take = read.take(start_rx_bytes(self.format) as u64);
        let rx_bytes = take.read_to_end(&mut self.frame_data)?;

        if rx_bytes != start_rx_bytes(self.format) {
            return Err(RxError::PreambleTruncated);
        }

        self.handle_pre_data()
    }
}

impl<'buf> RxFrame<'buf, ReadPreamble<'_>> {
    fn decrypt_block(&mut self) -> Result<(), RxError> {
        let Self {
            state, frame_data, ..
        } = self;

        let tag_len = state
            .encryption
            .decrypt(&mut frame_data[state.decrypted_len..])
            .map_err(|_| RxError::DecryptionFailed)?;

        let new_len = frame_data.len().checked_sub(tag_len).unwrap();
        frame_data.truncate(new_len);
        state.decrypted_len = frame_data.len();

        Ok(())
    }

    pub fn read_rest(
        mut self,
        mut read: impl std::io::Read,
    ) -> Result<RxFrame<'buf, Completed>, RxError> {
        let additional_blocks = rest_blocks(&self.state.preamble);

        for block in additional_blocks {
            let mut take = (&mut read).take(block as u64);
            let rx_bytes = take.read_to_end(self.frame_data)?;

            if rx_bytes != block {
                return Err(RxError::FrameDataTruncated);
            }

            self.decrypt_block()?;
        }

        Ok(RxFrame {
            state: Completed {
                preamble_data: self.state.preamble_data,
                preamble: self.state.preamble,
            },
            format: self.format,
            frame_data: self.frame_data,
        })
    }
}

impl RxFrame<'_, Completed> {
    pub(crate) fn preamble_data(&self) -> &[u8] {
        self.state.preamble_data.as_ref()
    }

    pub(crate) fn preamble(&self) -> &Preamble {
        &self.state.preamble
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.frame_data
    }
}

#[derive(Debug)]
pub struct TxFrame<'enc_buf> {
    pub(crate) preamble: Preamble,
    pub(crate) enc: &'enc_buf mut FrameEncryption,
    /// The unencrypted frame data (including prologue,
    /// padding, and epilogue)
    pub(crate) frame_data: &'enc_buf mut [u8],
}

impl TxFrame<'_> {
    pub fn write(self, mut output: impl std::io::Write) -> Result<usize, TxError> {
        match self.preamble.format {
            FrameFormat::Rev0Crc | FrameFormat::Rev1Crc => {
                output.write_all(self.frame_data)?;
                Ok(self.frame_data.len())
            }
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => {
                const LEN: usize = Preamble::SERIALIZED_SIZE + REV1_SECURE_INLINE_SIZE;
                let mut preamble = [0u8; LEN];
                let preamble_and_inline_data_len =
                    Preamble::SERIALIZED_SIZE + self.preamble.segments()[0].len();

                let len = LEN
                    .min(preamble_and_inline_data_len)
                    .min(self.frame_data.len());

                let (preamble_data, rest) = self.frame_data.split_at_mut(len);

                preamble[..len].copy_from_slice(preamble_data);

                let tag = self
                    .enc
                    .encrypt(preamble.as_mut_slice())
                    .map_err(|_| TxError::EncryptionFailed)?;

                output.write_all(preamble.as_slice())?;
                output.write_all(tag.as_slice())?;

                let rest_len = if !rest.is_empty() {
                    let tag = self
                        .enc
                        .encrypt(rest)
                        .map_err(|_| TxError::EncryptionFailed)?;

                    output.write_all(rest)?;
                    output.write_all(tag.as_slice())?;

                    tag.len() + rest.len()
                } else {
                    0
                };

                let total = LEN + AES_GCM_SIG_SIZE + rest_len;
                Ok(total)
            }
        }
    }
}
