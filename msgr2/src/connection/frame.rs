use std::io::Read;

use crate::{
    connection::encryption::FrameEncryption,
    frame::{FrameFormat, Preamble},
};

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

pub struct Unstarted<'enc> {
    pub(crate) encryption: &'enc mut FrameEncryption,
}

pub struct ReadPreamble<'enc> {
    pub(crate) encryption: &'enc mut FrameEncryption,
    pub(crate) preamble: Preamble,
    pub(crate) preamble_data: [u8; Preamble::SERIALIZED_SIZE],
    /// The index up to which data has been "treated" (= decrypted)
    pub(crate) treated_len: usize,
}

pub struct Completed {
    pub(crate) preamble_data: [u8; Preamble::SERIALIZED_SIZE],
    pub(crate) preamble: Preamble,
}

#[derive(Debug)]
pub struct RxFrame<'buf, T> {
    state: T,
    format: FrameFormat,
    /// The decrypted frame data
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
            .ok_or(RxError::DecryptionFailed)?;

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
                treated_len: frame_data.len(),
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
        let mut take = read.take(self.format.start_rx_bytes() as u64);
        let rx_bytes = take.read_to_end(&mut self.frame_data)?;

        if rx_bytes != self.format.start_rx_bytes() {
            return Err(RxError::PreambleTruncated);
        }

        self.handle_pre_data()
    }
}

impl<'buf> RxFrame<'buf, ReadPreamble<'_>> {
    fn handle_rest(self) -> Result<RxFrame<'buf, Completed>, RxError> {
        let Self {
            state,
            format,
            frame_data,
        } = self;

        let tag_len = state
            .encryption
            .decrypt(&mut frame_data[state.treated_len..])
            .ok_or(RxError::DecryptionFailed)?;

        let new_len = frame_data.len().checked_sub(tag_len).unwrap();
        frame_data.truncate(new_len);

        Ok(RxFrame {
            state: Completed {
                preamble_data: state.preamble_data,
                preamble: state.preamble,
            },
            format,
            frame_data,
        })
    }

    pub fn read_rest(self, read: impl std::io::Read) -> Result<RxFrame<'buf, Completed>, RxError> {
        let new_data_len = self.state.preamble.data_and_epilogue_len();
        let mut take = read.take(new_data_len);
        let rx_bytes = take.read_to_end(self.frame_data)?;

        if rx_bytes as u64 != new_data_len {
            return Err(RxError::FrameDataTruncated);
        }

        self.handle_rest()
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
pub struct TxFrame<'a> {
    pub(crate) data: &'a [u8],
}

impl TxFrame<'_> {
    pub fn write(self, mut output: impl std::io::Write) -> std::io::Result<usize> {
        output.write_all(&self.data)?;
        Ok(self.data.len())
    }
}
