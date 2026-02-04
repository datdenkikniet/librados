use std::collections::HashMap;

use ceph_foundation::{Decode, Timestamp, Uuid, entity::AddrVec, get_versions_and_data, pg::Pg};

use crate::{DecodeMessage, Epoch};

#[derive(Debug, Clone)]
pub struct MessageOsdMap {
    pub fsid: Uuid,
    pub incremental_maps: HashMap<Epoch, Vec<u8>>,
    pub maps: HashMap<Epoch, ByteArrayEncoded<OsdMap>>,
    pub cluster_osdmap_trim_lower_bound: Epoch,
    pub newest_map: Epoch,
}

impl DecodeMessage<'_> for MessageOsdMap {
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

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct PoolId(i64);

impl<'a> Decode<'a> for PoolId {
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, ceph_foundation::DecodeError> {
        Ok(Self(i64::decode(buffer)?))
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct PoolMax(i32);

impl<'a> Decode<'a> for PoolMax {
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, ceph_foundation::DecodeError> {
        Ok(Self(i32::decode(buffer)?))
    }
}

#[derive(Debug, Clone)]
pub struct OsdMap {
    pub fsid: Uuid,
    pub epoch: Epoch,
    pub created: Timestamp,
    pub modified: Timestamp,
    pub pool_name: HashMap<PoolId, String>,
    pub pool_max: PoolMax,
}

impl<'a> Decode<'a> for OsdMap {
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, ceph_foundation::DecodeError> {
        let (_version, mut buffer) = get_versions_and_data!(OsdMap: buffer, 7);
        let buffer = &mut buffer;

        // Client-usable data
        let osd_map = {
            let (_inner_version, mut buffer) = get_versions_and_data!(OsdMap: buffer, 9, 6);
            let buffer = &mut buffer;
            let fsid = Uuid::decode(buffer)?;
            let epoch = Epoch::decode(buffer)?;
            let created = Timestamp::decode(buffer)?;
            let modified = Timestamp::decode(buffer)?;
            let _pools = HashMap::<PoolId, SkipWithLength>::decode(buffer)?;
            let pool_name = HashMap::<PoolId, String>::decode(buffer)?;
            let pool_max = PoolMax::decode(buffer)?;
            let flags = u32::decode(buffer)?;
            let max_osd = i32::decode(buffer)?;

            // Only starting at version 5, but min we support is 6
            let osd_state: Vec<u32> = Vec::decode(buffer)?;
            let osd_weight: Vec<u32> = Vec::decode(buffer)?;
            let client_addrs: Vec<AddrVec> = Vec::decode(buffer)?;

            let pg_temp_map: HashMap<Pg, u32> = HashMap::decode(buffer)?;

            Self {
                fsid,
                epoch,
                created,
                modified,
                pool_name,
                pool_max,
            }
        };

        {
            // OSD-only data
            let (_version, _buffer) = get_versions_and_data!(OsdMap: buffer, u8::MAX);
        }

        Ok(osd_map)
    }
}

/// Wrapper meant for types that ceph library can express as an array of bytes
///
/// ceph/src/messages/MOSDMap.h:34 is an example
#[derive(Debug, Clone)]
pub struct ByteArrayEncoded<T>(T);

impl<'a, T: Decode<'a>> Decode<'a> for ByteArrayEncoded<T> {
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, ceph_foundation::DecodeError> {
        // consume length
        let buffer = &mut <&[u8]>::decode(buffer)?;
        Ok(Self(T::decode(buffer)?))
    }
}

pub struct SkipWithLength;

impl<'a> Decode<'a> for SkipWithLength {
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, ceph_foundation::DecodeError> {
        // ignore versions
        <[u8; 2]>::decode(buffer)?;
        <&[u8]>::decode(buffer)?;
        Ok(Self)
    }
}
