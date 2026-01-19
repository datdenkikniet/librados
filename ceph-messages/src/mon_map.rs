use std::collections::{HashMap, HashSet};

use ceph_foundation::{
    Decode, DecodeError, Encode, Encoder, MonInfo, Timestamp, Uuid, write_decode_encode,
};

use crate::DecodeMessage;

#[derive(Debug, Clone)]
pub struct MonMap {}

impl DecodeMessage<'_> for MonMap {
    fn decode_message(data_segments: &[&'_ [u8]]) -> Result<Self, crate::DecodeMessageError> {
        let mut data_segment = data_segments[0];
        let [version, compat] = <[u8; 2]>::decode(&mut data_segment)?;

        if version != 9 || compat != 6 {
            return Err(DecodeError::UnexpectedVersion {
                ty: "MonMap",
                got: version,
                expected: 9..=9,
            }
            .into());
        }

        let mut map_data = <&[u8]>::decode(&mut data_segment)?;
        let map = MonMap9_6::decode(&mut map_data)?;
        Ok(map.into())
    }
}

#[derive(Clone, Debug)]
struct MonMap9_6 {
    epoch: u32,
    fsid: Uuid,
    last_changed: Timestamp,
    created: Timestamp,
    mon_info: HashMap<String, MonInfo>,
    ranks: Vec<String>,
    removed_ranks: Vec<u32>,
    persistent_features: MonFeatures,
    optional_features: MonFeatures,
    min_mon_release: [u8; 1],
    strategy: [u8; 1],
    disallowed_leaders: HashSet<String>,
    stretch_mode_enabled: bool,
    tiebreaker_mon: String,
    stretch_marked_down_mons: HashSet<String>,
}

write_decode_encode!(
    MonMap9_6 = fsid
        | epoch
        | last_changed
        | created
        | persistent_features
        | optional_features
        | mon_info
        | ranks
        | min_mon_release
        | removed_ranks
        | strategy
        | disallowed_leaders
        | stretch_mode_enabled
        | tiebreaker_mon
        | stretch_marked_down_mons
);

impl Into<MonMap> for MonMap9_6 {
    fn into(self) -> MonMap {
        MonMap {}
    }
}

#[derive(Clone, Debug)]
pub struct MonFeatures {
    value: u64,
}

impl Encode for MonFeatures {
    fn encode(&self, buffer: &mut impl Encoder) {
        [1u8, 1].encode(buffer);
        // This struct requires us to encode the length...
        // Find a better way to deal with this (hopefully
        // with the macro)
        8u32.encode(buffer);
        self.value.encode(buffer);
    }
}

impl Decode<'_> for MonFeatures {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let [ver, compat_ver] = <[u8; 2]>::decode(buffer)?;

        if ver != 1 {
            return Err(DecodeError::UnexpectedVersion {
                ty: "MonFeatures",
                got: ver,
                expected: 1..=1,
            });
        }

        if compat_ver != 1 {
            return Err(DecodeError::UnexpectedVersion {
                ty: "MonFeatures",
                got: ver,
                expected: 1..=1,
            });
        }

        let mut buffer = <&[u8]>::decode(buffer)?;
        let value = u64::decode(&mut buffer)?;

        Ok(Self { value })
    }
}

#[test]
fn mon_map_9_6() {
    #[rustfmt::skip]
    let data = [
        9, 6, // Version data
        210, 0, 0, 0, // Len
        213, 24, 184, 84, 231, 33, 17, 240, 137, 38, 188, 36, 17, 128, 136, 187, // UUID
        1, 0, 0, 0, // Epoch
        255, 138, 86, 105, 208, 152, 56, 40, // Last changed
        255, 138, 86, 105, 208, 152, 56, 40, // Created
        1, 1, 8, 0, 0, 0, 255, 3, 0, 0, 0, 0, 0, 0, // persistent features
        1, 1, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // Required features
        1, 0, 0, 0, // Entry len
            6, 0, 0, 0, 99, 101, 112, 104, 48, 49, // Key (String)
            5, 1, 93, 0, 0, 0, 6, 0, 0, 0, 99, 101, 112,
        104, 48, 49, 2, 2, 0, 0, 0, 1, 1, 1, 28, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 2,
        0, 12, 228, 10, 0, 1, 222, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 28, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 16, 0, 0, 0, 2, 0, 26, 133, 10, 0, 1, 222, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 6, 0, 0, 0, 99, 101, 112, 104, 48, 49, 18, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let mon_map = MonMap::decode_message(&[&data]).unwrap();

    panic!("{mon_map:#?}");
}
