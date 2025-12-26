#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CephFeatures {
    bits: u64,
    mask: u64,
}

macro_rules ! define_features {
    ($(($(qual:tt:)? $bit:literal, $incarnation: ident, $name:ident, $mask:ident)),*$(,)?) => {
        impl CephFeatures {
            $(
                feature!($(qual:)? $bit, $incarnation, $name, $mask);
            )*

            pub const ALL: CephFeatures = CephFeatures::EMPTY $(| CephFeatures::$name)*;
        }
    }
}

macro_rules! feature {
    ($bit:literal, $incarnation:ident, $name:ident, $mask:ident) => {
        pub const $name: CephFeatures = CephFeatures {
            bits: 1 << $bit,
            mask: 1 << $bit | Self::$incarnation,
        };
    };

    (deprecated: $bit:literal, $incarnation:ident, $name:ident, $mask:ident) => {
        feature!($bit, $incarnation, $name, $mask)
    };

    (retired: $bit:literal, $incarnation:ident, $name:ident) => {
        #[allow(unused)]
        const $name: CephFeatures = CephFeatures::EMPTY;
    };
}

#[rustfmt::skip]
impl CephFeatures {
    feature!( 0, INCARNATION_1, UID, UID_MASK);
    feature!( 1, INCARNATION_1, NOSRCADDR, NOSRCADDR_MASK);        // 2.6.35 req
    feature!(retired:  2, INCARNATION_1, MONCLOCKCHECK);
    feature!( 2, INCARNATION_3, SERVER_NAUTILUS, SERVER_NAUTILUS_MASK);
    feature!( 3, INCARNATION_1, FLOCK, FLOCK_MASK);            // 2.6.36
    feature!( 4, INCARNATION_1, SUBSCRIBE2, SUBSCRIBE2_MASK);       // 4.6 req
    feature!( 5, INCARNATION_1, MONNAMES, MONNAMES_MASK);
    feature!( 6, INCARNATION_1, RECONNECT_SEQ, RECONNECT_SEQ_MASK);    // 3.10 req
    feature!( 7, INCARNATION_1, DIRLAYOUTHASH, DIRLAYOUTHASH_MASK);    // 2.6.38
    feature!( 8, INCARNATION_1, OBJECTLOCATOR, OBJECTLOCATOR_MASK);
    feature!( 9, INCARNATION_1, PGID64, PGID64_MASK);           // 3.9 req
    feature!(10, INCARNATION_1, INCSUBOSDMAP, INCSUBOSDMAP_MASK);
    feature!(11, INCARNATION_1, PGPOOL3, PGPOOL3_MASK);          // 3.9 req
    feature!(12, INCARNATION_1, OSDREPLYMUX, OSDREPLYMUX_MASK);
    feature!(13, INCARNATION_1, OSDENC, OSDENC_MASK);           // 3.9 req
    feature!(retired: 14, INCARNATION_1, OMAP);
    feature!(14, INCARNATION_2, SERVER_KRAKEN, SERVER_KRAKEN_MASK);
    feature!(15, INCARNATION_1, MONENC, MONENC_MASK);
    feature!(retired: 16, INCARNATION_1, QUERY_T);
    feature!(16, INCARNATION_3, SERVER_OCTOPUS, SERVER_OCTOPUS_MASK);
    feature!(16, INCARNATION_3, OSD_REPOP_MLCOD, OSD_REPOP_MLCOD_MASK);
    feature!(retired: 17, INCARNATION_1, INDEP_PG_MAP);
    feature!(17, INCARNATION_3, OS_PERF_STAT_NS, OS_PERF_STAT_NS_MASK);
    feature!(18, INCARNATION_1, CRUSH_TUNABLES, CRUSH_TUNABLES_MASK);   // 3.6
    feature!(retired: 19, INCARNATION_1, CHUNKY_SCRUB);
    feature!(19, INCARNATION_2, OSD_PGLOG_HARDLIMIT, OSD_PGLOG_HARDLIMIT_MASK);
    feature!(retired: 20, INCARNATION_1, MON_NULLROUTE);
    feature!(20, INCARNATION_3, SERVER_PACIFIC, SERVER_PACIFIC_MASK);
    feature!(retired: 21, INCARNATION_1, MON_GV);
    feature!(21, INCARNATION_2, SERVER_LUMINOUS, SERVER_LUMINOUS_MASK);  // 4.13
    feature!(21, INCARNATION_2, RESEND_ON_SPLIT, RESEND_ON_SPLIT_MASK);  // overlap
    feature!(21, INCARNATION_2, RADOS_BACKOFF, RADOS_BACKOFF_MASK);    // overlap
    feature!(21, INCARNATION_2, OSDMAP_PG_UPMAP, OSDMAP_PG_UPMAP_MASK);  // overlap
    feature!(21, INCARNATION_2, CRUSH_CHOOSE_ARGS, CRUSH_CHOOSE_ARGS_MASK); // overlap
    feature!(retired: 22, INCARNATION_1, BACKFILL_RESERVATION);
    feature!(22, INCARNATION_2, OSD_FIXED_COLLECTION_LIST, OSD_FIXED_COLLECTION_LIST_MASK);
    feature!(23, INCARNATION_1, MSG_AUTH, MSG_AUTH_MASK);         // 3.19 req (unless nocephx_require_signatures);
    feature!(retired: 24, INCARNATION_1, RECOVERY_RESERVATION);
    feature!(24, INCARNATION_2, RECOVERY_RESERVATION_2, RECOVERY_RESERVATION_2_MASK);
    feature!(25, INCARNATION_1, CRUSH_TUNABLES2, CRUSH_TUNABLES2_MASK);  // 3.9
    feature!(26, INCARNATION_1, CREATEPOOLID, CREATEPOOLID_MASK);
    feature!(27, INCARNATION_1, REPLY_CREATE_INODE, REPLY_CREATE_INODE_MASK); // 3.9
    feature!(retired: 28, INCARNATION_1, OSD_HBMSGS);
    feature!(28, INCARNATION_2, SERVER_MIMIC, SERVER_MIMIC_MASK);
    feature!(29, INCARNATION_1, MDSENC, MDSENC_MASK);           // 4.7
    feature!(30, INCARNATION_1, OSDHASHPSPOOL, OSDHASHPSPOOL_MASK);    // 3.9
    feature!(retired: 31, INCARNATION_1, MON_SINGLE_PAXOS);
    feature!(31, INCARNATION_3, SERVER_REEF, SERVER_REEF_MASK);
    feature!(retired: 32, INCARNATION_1, OSD_SNAPMAPPER);
    feature!(32, INCARNATION_3, STRETCH_MODE, STRETCH_MODE_MASK);
    feature!(retired: 33, INCARNATION_1, MON_SCRUB);
    feature!(33, INCARNATION_3, SERVER_QUINCY, SERVER_QUINCY_MASK);
    feature!(retired: 34, INCARNATION_1, OSD_PACKED_RECOVERY);
    feature!(34, INCARNATION_3, RANGE_BLOCKLIST, RANGE_BLOCKLIST_MASK);
    feature!(35, INCARNATION_1, OSD_CACHEPOOL, OSD_CACHEPOOL_MASK);    // 3.14
    feature!(36, INCARNATION_1, CRUSH_V2, CRUSH_V2_MASK);         // 3.14
    feature!(37, INCARNATION_1, EXPORT_PEER, EXPORT_PEER_MASK);      // 3.14
    feature!(38, INCARNATION_2, CRUSH_MSR, CRUSH_MSR_MASK);        // X.XX kernel version once in a release
    feature!(39, INCARNATION_1, OSDMAP_ENC, OSDMAP_ENC_MASK);       // 3.15
    feature!(40, INCARNATION_1, MDS_INLINE_DATA, MDS_INLINE_DATA_MASK);  // 3.19
    feature!(41, INCARNATION_1, CRUSH_TUNABLES3, CRUSH_TUNABLES3_MASK);  // 3.15
    feature!(41, INCARNATION_1, OSD_PRIMARY_AFFINITY, OSD_PRIMARY_AFFINITY_MASK); // overlap
    feature!(42, INCARNATION_1, MSGR_KEEPALIVE2, MSGR_KEEPALIVE2_MASK);  // 4.3 (for consistency);
    feature!(43, INCARNATION_1, OSD_POOLRESEND, OSD_POOLRESEND_MASK);   // 4.13
    feature!(44, INCARNATION_2, NVMEOFHA, NVMEOFHA_MASK);
    feature!(retired: 45, INCARNATION_1, OSD_SET_ALLOC_HINT);
    feature!(45, INCARNATION_2, NVMEOFHAMAP, NVMEOFHAMAP_MASK);
    // available
    feature!(46, INCARNATION_1, OSD_FADVISE_FLAGS, OSD_FADVISE_FLAGS_MASK);
    feature!(retired: 46, INCARNATION_1, OSD_REPOP); // overlap
    feature!(retired: 46, INCARNATION_1, OSD_OBJECT_DIGEST); // overlap
    feature!(retired: 46, INCARNATION_1, OSD_TRANSACTION_MAY_LAYOUT); // overlap
    feature!(47, INCARNATION_1, MDS_QUOTA, MDS_QUOTA_MASK);        // 4.17
    feature!(48, INCARNATION_1, CRUSH_V4, CRUSH_V4_MASK);         // 4.1
    feature!(retired: 49, INCARNATION_1, OSD_MIN_SIZE_RECOVERY);
    feature!(retired: 49, INCARNATION_1, OSD_PROXY_FEATURES); // overlap
    feature!(49, INCARNATION_2, SERVER_SQUID, SERVER_SQUID_MASK);
    feature!(retired: 50, INCARNATION_1, MON_METADATA);
    feature!(50, INCARNATION_2, SERVER_TENTACLE, SERVER_TENTACLE_MASK);
    feature!(retired: 51, INCARNATION_1, OSD_BITWISE_HOBJ_SORT);
    // available
    feature!(retired: 52, INCARNATION_1, OSD_PROXY_WRITE_FEATURES);
    // available
    feature!(retired: 53, INCARNATION_1, ERASURE_CODE_PLUGINS_V3);
    // available
    feature!(retired: 54, INCARNATION_1, OSD_HITSET_GMT);
    // available
    feature!(retired: 55, INCARNATION_1, HAMMER_0_94_4);
    // available
    feature!(56, INCARNATION_1, NEW_OSDOP_ENCODING, NEW_OSDOP_ENCODING_MASK); // 4.13 (for pg_pool_t >= v25);
    feature!(57, INCARNATION_1, MON_STATEFUL_SUB, MON_STATEFUL_SUB_MASK); // 4.13
    feature!(retired: 57, INCARNATION_1, MON_ROUTE_OSDMAP); // overlap
    feature!(57, INCARNATION_1, SERVER_JEWEL, SERVER_JEWEL_MASK); // overlap
    feature!(58, INCARNATION_1, CRUSH_TUNABLES5, CRUSH_TUNABLES5_MASK);  // 4.5
    feature!(58, INCARNATION_1, NEW_OSDOPREPLY_ENCODING, NEW_OSDOPREPLY_ENCODING_MASK); // overlap
    feature!(58, INCARNATION_1, FS_FILE_LAYOUT_V2, FS_FILE_LAYOUT_V2_MASK); // overlap
    feature!(59, INCARNATION_1, FS_BTIME, FS_BTIME_MASK);
    feature!(59, INCARNATION_1, FS_CHANGE_ATTR, FS_CHANGE_ATTR_MASK); // overlap
    feature!(59, INCARNATION_1, MSG_ADDR2, MSG_ADDR2_MASK); // overlap
    feature!(60, INCARNATION_1, OSD_RECOVERY_DELETES, OSD_RECOVERY_DELETES_MASK); // *do not share this bit*
    feature!(61, INCARNATION_1, CEPHX_V2, CEPHX_V2_MASK);         // 4.19, *do not share this bit*

    feature!(62, INCARNATION_1, RESERVED, RESERVED_MASK);           // do not use; used as a sentinel
    feature!(retired: 63, INCARNATION_1, RESERVED_BROKEN); // client-facing
    // available
}

impl CephFeatures {
    const INCARNATION_1: u64 = 0;
    const INCARNATION_2: u64 = 1 << 57;
    const INCARNATION_3: u64 = 1 << 57 | 1 << 28;
    pub const EMPTY: CephFeatures = CephFeatures {
        bits: 0,
        mask: 0,
    };

    pub fn get(&self) -> u64 {
        self.bits
    }

    pub fn has(&self, features: CephFeatures) -> bool {
        self.bits & features.mask == features.mask
    }
}

impl core::ops::BitOr for CephFeatures {
    type Output = CephFeatures;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
            mask: self.mask | rhs.mask,
        }
    }
}

impl core::ops::BitOrAssign for CephFeatures {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl core::ops::BitAnd for CephFeatures {
    type Output = CephFeatures;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
            mask: self.mask & rhs.mask,
        }
    }
}

impl core::ops::BitAndAssign for CephFeatures {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

#[test]
fn has_combination() {
    let f1 = CephFeatures::UID;
    let f2 = CephFeatures::MONNAMES;
    let f3 = CephFeatures::CREATEPOOLID;
    let combined = f1 | f2;

    assert!(combined.has(f1));
    assert!(combined.has(f2));
    assert!(!combined.has(f3));
}
