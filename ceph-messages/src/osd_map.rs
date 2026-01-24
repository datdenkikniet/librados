use std::collections::HashMap;

use ceph_foundation::{Decode, Uuid};

use crate::{DecodeMessage, Epoch};

#[derive(Debug, Clone)]
pub struct OsdMap {
    pub fsid: Uuid,
    pub incremental_maps: HashMap<Epoch, Vec<u8>>,
    pub maps: HashMap<Epoch, Vec<u8>>,
    pub cluster_osdmap_trim_lower_bound: Epoch,
    pub newest_map: Epoch,
}

impl DecodeMessage<'_> for OsdMap {
    fn decode_message(segments: &[&'_ [u8]]) -> Result<Self, crate::DecodeMessageError> {
        if segments.len() > 1 {
            return Err(crate::DecodeMessageError::TooManySegments {
                have: segments.len(),
                want: 1,
            });
        } else if segments.is_empty() {
            return Err(crate::DecodeMessageError::NotEnoughSegments { have: 0, need: 1 });
        }

        let buffer = &mut segments[0].as_ref();
        let fsid = Uuid::decode(buffer)?;
        let incremental_maps = Decode::decode(buffer)?;
        let maps = Decode::decode(buffer)?;
        let cluster_osdmap_trim_lower_bound = Epoch::decode(buffer)?;
        let newest_map = Epoch::decode(buffer)?;

        Ok(Self {
            fsid,
            incremental_maps,
            maps,
            cluster_osdmap_trim_lower_bound,
            newest_map,
        })
    }
}
