use std::collections::{HashMap, HashSet};

use ceph_foundation::{Decode, DecodeError, Encode, Encoder, MonInfo, Timestamp, Uuid, WireString};

use crate::DecodeMessage;

#[derive(Debug, Clone, PartialEq)]
pub struct MonMap {
    pub epoch: u32,
    pub fsid: Uuid,
    pub last_changed: Timestamp,
    pub created: Timestamp,
    pub mon_info: HashMap<String, MonInfo>,
    pub ranks: Vec<String>,
    pub removed_ranks: Vec<u32>,
    pub persistent_features: MonFeatures,
    pub optional_features: MonFeatures,
    pub min_mon_release: [u8; 1],
    pub strategy: [u8; 1],
    pub disallowed_leaders: HashSet<String>,
    pub stretch_mode_enabled: bool,
    pub tiebreaker_mon: String,
    pub stretch_marked_down_mons: HashSet<String>,
}

impl DecodeMessage<'_> for MonMap {
    fn decode_message(data_segments: &[&'_ [u8]]) -> Result<Self, crate::DecodeMessageError> {
        let mut data_segment = data_segments[0];
        let (version, mut data) =
            ceph_foundation::get_versions_and_data!(MonMap: &mut data_segment, 9, 6);

        let data = &mut data;
        let fsid = Uuid::decode(data)?;
        let epoch = u32::decode(data)?;
        let last_changed = Timestamp::decode(data)?;
        let created = Timestamp::decode(data)?;

        // Both of these only for version >= 4, but min we support is 6
        let persistent_features = MonFeatures::decode(data)?;
        let optional_features = MonFeatures::decode(data)?;

        // Only for version >= 5, but min we support is 6
        let mon_info = Decode::decode(data)?;

        // Only for version >= 6, but min we support is 6
        let ranks = Decode::decode(data)?;

        // TODO: not `default` here?
        let min_mon_release = Decode::decode_if(version >= 7, data)?
            .unwrap_or_else(|| todo!("Infer mon release from features"));

        let (removed_ranks, strategy, disallowed_leaders) = if version >= 8 {
            (
                Decode::decode(data)?,
                Decode::decode(data)?,
                Decode::decode(data)?,
            )
        } else {
            (Default::default(), Default::default(), Default::default())
        };

        let (stretch_mode_enabled, tiebreaker_mon, stretch_marked_down_mons) = if version >= 9 {
            (
                Decode::decode(data)?,
                WireString::decode(data)?,
                Decode::decode(data)?,
            )
        } else {
            (false, WireString::default(), Default::default())
        };

        Ok(Self {
            epoch,
            fsid,
            last_changed,
            created,
            mon_info,
            ranks,
            removed_ranks,
            persistent_features,
            optional_features,
            min_mon_release,
            strategy,
            disallowed_leaders,
            stretch_mode_enabled,
            tiebreaker_mon: tiebreaker_mon.into(),
            stretch_marked_down_mons,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MonFeatures {
    value: u64,
}

impl Encode for MonFeatures {
    fn encode(&self, buffer: &mut impl Encoder) {
        let buffer = &mut ceph_foundation::write_versions_and_data!(buffer, 1, 1);
        self.value.encode(buffer);
    }
}

impl Decode<'_> for MonFeatures {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let (_version, mut buffer) =
            ceph_foundation::get_versions_and_data!(MonFeatures: buffer, 1);
        let value = u64::decode(&mut buffer)?;
        Ok(Self { value })
    }
}

#[test]
fn encode_mon_features() {
    let features = MonFeatures { value: 0xAABBCC };
    let encoded = features.to_vec();

    assert_eq!(
        encoded.as_slice(),
        &[1, 1, 8, 0, 0, 0, 0xCC, 0xBB, 0xAA, 0, 0, 0, 0, 0]
    );
}

#[test]
fn mon_map_9_6() {
    use ceph_foundation::entity::{EntityAddress, EntityAddressType};
    use std::net::{Ipv4Addr, SocketAddrV4};

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

    let ip = Ipv4Addr::new(10, 0, 1, 222);
    let addr = EntityAddress {
        ty: EntityAddressType::Msgr2,
        nonce: 0,
        address: Some(SocketAddrV4::new(ip, 3300).into()),
    };

    let legacy_addr = EntityAddress {
        ty: EntityAddressType::Legacy,
        nonce: 0,
        address: Some(SocketAddrV4::new(ip, 6789).into()),
    };

    let mon_info = MonInfo {
        name: "ceph01".into(),
        public_addrs: vec![addr, legacy_addr],
        priority: 0,
        weight: 0,
        crush_location: Default::default(),
        // The encoded MonInfo is version 5, which does
        // not include this field yet.
        time_added: Default::default(),
    };

    let mon_info = [("ceph01".to_string(), mon_info)].into_iter().collect();

    let expected = MonMap {
        epoch: 1,
        fsid: Uuid([
            213, 24, 184, 84, 231, 33, 17, 240, 137, 38, 188, 36, 17, 128, 136, 187,
        ]),
        last_changed: Timestamp::new(1767279359, 674797776),
        created: Timestamp::new(1767279359, 674797776),
        mon_info,
        ranks: vec!["ceph01".to_string()],
        removed_ranks: Vec::new(),
        persistent_features: MonFeatures { value: 1023 },
        optional_features: MonFeatures { value: 0 },
        min_mon_release: [18],
        strategy: [1],
        disallowed_leaders: HashSet::new(),
        stretch_mode_enabled: false,
        tiebreaker_mon: "".to_string(),
        stretch_marked_down_mons: HashSet::new(),
    };

    assert_eq!(mon_map, expected);
}
