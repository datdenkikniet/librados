use std::io::Read;

use crate::{
    connection::encryption::FrameEncryption,
    frame::{FrameFormat, Preamble, REV1_SECURE_INLINE_SIZE, REV1_SECURE_PAD_SIZE},
    key::AES_GCM_SIG_SIZE,
};

fn start_bytes(format: FrameFormat) -> usize {
    match format {
        FrameFormat::Rev0Crc => crate::frame::Preamble::SERIALIZED_SIZE,
        FrameFormat::Rev1Crc => crate::frame::Preamble::SERIALIZED_SIZE,
        FrameFormat::Rev0Secure => todo!(),
        FrameFormat::Rev1Secure => Preamble::SERIALIZED_SIZE + REV1_SECURE_INLINE_SIZE,
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
                let non_trailer_data = preamble.segments()[0].len().min(REV1_SECURE_INLINE_SIZE);
                frame_data.truncate(non_trailer_data);

                // If there is no more data to be read for the frame, insert artificial
                // padding data: `crate::frame::Frame` expects (and inserts)
                // alignment padding for all data, while inlining is only done
                // by `RxFrame` and `TxFrame`.
                if frame_data.len() <= REV1_SECURE_INLINE_SIZE {
                    let padded_len = frame_data
                        .len()
                        .next_multiple_of(REV1_SECURE_PAD_SIZE.get());
                    let pad_required = padded_len - frame_data.len();

                    frame_data.extend(core::iter::repeat_n(0, pad_required));
                }
            }
        }

        Ok(RxFrame {
            state: ReadPreamble {
                preamble_data,
                encryption: encryption,
                preamble,
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
        let start_rx_bytes = match self.format {
            FrameFormat::Rev0Crc | FrameFormat::Rev1Crc => start_bytes(self.format),
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => start_bytes(self.format) + AES_GCM_SIG_SIZE,
        };

        let mut take = read.take(start_rx_bytes as u64);
        let rx_bytes = take.read_to_end(&mut self.frame_data)?;

        if rx_bytes != start_rx_bytes {
            return Err(RxError::PreambleTruncated);
        }

        self.handle_pre_data()
    }
}

impl<'buf> RxFrame<'buf, ReadPreamble<'_>> {
    fn decrypt_block(&mut self, new_data_start: usize) -> Result<(), RxError> {
        let Self {
            state, frame_data, ..
        } = self;

        let tag_len = state
            .encryption
            .decrypt(&mut frame_data[new_data_start..])
            .map_err(|_| RxError::DecryptionFailed)?;

        let new_len = frame_data.len().checked_sub(tag_len).unwrap();
        frame_data.truncate(new_len);

        // No need to insert artificial padding here:

        Ok(())
    }

    pub fn read_rest(
        mut self,
        mut read: impl std::io::Read,
    ) -> Result<RxFrame<'buf, Completed>, RxError> {
        let additional_blocks = self.state.preamble.data_and_epilogue_segments();

        for block in additional_blocks {
            let new_data_start = self.frame_data.len();

            let block = match self.format {
                FrameFormat::Rev0Crc | FrameFormat::Rev1Crc => block,
                FrameFormat::Rev0Secure => todo!(),
                FrameFormat::Rev1Secure => block + AES_GCM_SIG_SIZE,
            };

            let mut take = (&mut read).take(block as u64);
            let rx_bytes = take.read_to_end(self.frame_data)?;

            if rx_bytes != block {
                return Err(RxError::FrameDataTruncated);
            }

            self.decrypt_block(new_data_start)?;
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

                let (preamble_data, mut rest) = self.frame_data.split_at_mut(len);

                preamble[..len].copy_from_slice(preamble_data);

                let tag = self
                    .enc
                    .encrypt(preamble.as_mut_slice())
                    .map_err(|_| TxError::EncryptionFailed)?;

                output.write_all(preamble.as_slice())?;
                output.write_all(tag.as_slice())?;

                let mut total = LEN + tag.len();
                let rest_blocks = self.preamble.data_and_epilogue_segments();

                for block in rest_blocks {
                    let (block, new_rest) = rest
                        .split_at_mut_checked(block)
                        .expect("Preamble does not describe frame data.");

                    let tag = self
                        .enc
                        .encrypt(block)
                        .map_err(|_| TxError::EncryptionFailed)?;

                    output.write_all(block)?;
                    output.write_all(tag.as_slice())?;

                    total += block.len() + AES_GCM_SIG_SIZE;

                    rest = new_rest;
                }

                Ok(total)
            }
        }
    }
}
