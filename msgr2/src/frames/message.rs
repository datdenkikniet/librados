use ceph_foundation::DecodeError;

use crate::{Frame, Tag};

pub struct Message<'a> {
    header: &'a [u8],
    front: Option<&'a [u8]>,
    middle: Option<&'a [u8]>,
    back: Option<&'a [u8]>,
}

impl<'a> Message<'a> {
    pub fn new(header: &'a [u8], others: &[&'a [u8]]) -> Option<Self> {
        if others.len() > 3 {
            return None;
        }

        Some(Self {
            header,
            front: others.get(0).map(|v| *v),
            middle: others.get(1).map(|v| *v),
            back: others.get(2).map(|v| *v),
        })
    }

    pub fn decode(frame: &'a Frame<'a>) -> Result<Self, DecodeError> {
        let mut segments = frame.segments().iter().copied();
        let Some(header) = segments.next() else {
            return Err(DecodeError::Custom(
                "Received message frame without header segment".to_string(),
            ));
        };

        Ok(Message {
            header,
            front: segments.next(),
            middle: segments.next(),
            back: segments.next(),
        })
    }

    pub fn to_frame(&self) -> Frame<'a> {
        let segments = match (self.front, self.middle, self.back) {
            (None, None, None) => &[self.header][..],
            (Some(f), None, None) => &[self.header, f][..],
            (Some(f), Some(m), None) => &[self.header, f, m][..],
            (Some(f), Some(m), Some(b)) => &[self.header, f, m, b][..],
            (f, m, b) => unreachable!("{}, {}, {}", f.is_some(), m.is_some(), b.is_some()),
        };

        Frame::new(Tag::Message, segments).unwrap()
    }

    pub fn push_data_segment(&mut self, segment: &'a [u8]) -> bool {
        if self.front.is_none() {
            self.front = Some(segment);
            true
        } else if self.middle.is_none() {
            self.middle = Some(segment);
            true
        } else if self.back.is_none() {
            self.back = Some(segment);
            true
        } else {
            false
        }
    }

    pub fn pop_data_segment(&mut self) -> Option<&'a [u8]> {
        if let Some(b) = self.back.take() {
            Some(b)
        } else if let Some(m) = self.middle.take() {
            Some(m)
        } else if let Some(f) = self.front.take() {
            Some(f)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&[u8]> {
        self.front
    }

    pub fn middle(&self) -> Option<&[u8]> {
        self.middle
    }

    pub fn back(&self) -> Option<&[u8]> {
        self.back
    }

    pub fn data_segments(&self) -> impl Iterator<Item = &[u8]> {
        [self.front, self.middle, self.back].into_iter().flatten()
    }
}
