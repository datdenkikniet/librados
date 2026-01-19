use ceph_foundation::DecodeError;

use crate::{Frame, Tag};

#[derive(Debug)]
pub struct Message<'a> {
    header: &'a [u8],
    len: usize,
    inner: [&'a [u8]; 3],
}

impl<'a> Message<'a> {
    pub fn new(header: &'a [u8], others: &[&'a [u8]]) -> Result<Self, DecodeError> {
        if others.len() > 3 {
            return Err(DecodeError::Custom(format!(
                "Expected at most 3 data segments, got {}",
                others.len()
            )));
        }

        let mut inner = [&[][..]; 3];
        inner[..others.len()].copy_from_slice(others);

        Ok(Self {
            header,
            len: others.len(),
            inner,
        })
    }

    pub fn from_frame(frame: &'a Frame<'a>) -> Result<Self, DecodeError> {
        let segments = frame.segments();
        let (header, rest) = segments.split_first().ok_or_else(|| {
            DecodeError::Custom("Received message frame without header segment".to_string())
        })?;

        Self::new(header, rest)
    }

    pub fn to_frame(&self) -> Frame<'a> {
        Frame::new(Tag::Message, self.data_segments()).unwrap()
    }

    pub fn push_data_segment(&mut self, segment: &'a [u8]) -> bool {
        if self.len < 3 {
            self.inner[self.len] = segment;
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub fn pop_data_segment(&mut self) -> Option<&'a [u8]> {
        if let Some(len) = self.len.checked_sub(1) {
            self.len = len;
            Some(&self.inner[self.len])
        } else {
            None
        }
    }

    pub fn header(&self) -> &'a [u8] {
        self.header
    }

    pub fn front(&self) -> Option<&[u8]> {
        self.inner.get(0).map(|v| *v)
    }

    pub fn middle(&self) -> Option<&[u8]> {
        self.inner.get(1).map(|v| *v)
    }

    pub fn back(&self) -> Option<&'a [u8]> {
        self.inner.get(2).map(|v| *v)
    }

    pub fn data_segments(&self) -> &[&'a [u8]] {
        &self.inner[..self.len]
    }
}
