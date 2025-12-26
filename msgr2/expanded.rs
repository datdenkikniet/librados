#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod ceph_features {
    use crate::Encode;
    pub struct CephFeatureSet {
        pub(crate) bits: u64,
        pub(crate) mask: u64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CephFeatureSet {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CephFeatureSet",
                "bits",
                &self.bits,
                "mask",
                &&self.mask,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CephFeatureSet {
        #[inline]
        fn clone(&self) -> CephFeatureSet {
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CephFeatureSet {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CephFeatureSet {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CephFeatureSet {
        #[inline]
        fn eq(&self, other: &CephFeatureSet) -> bool {
            self.bits == other.bits && self.mask == other.mask
        }
    }
    impl Encode for CephFeatureSet {
        fn encode(&self, buffer: &mut Vec<u8>) {
            self.bits.encode(buffer);
        }
    }
    impl CephFeatureSet {
        pub const UID: CephFeatureSet = CephFeatureSet {
            bits: 1 << 0,
            mask: 1 << 0 | Self::INCARNATION_1,
        };
        pub const NOSRCADDR: CephFeatureSet = CephFeatureSet {
            bits: 1 << 1,
            mask: 1 << 1 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const MONCLOCKCHECK: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_NAUTILUS: CephFeatureSet = CephFeatureSet {
            bits: 1 << 2,
            mask: 1 << 2 | Self::INCARNATION_3,
        };
        pub const FLOCK: CephFeatureSet = CephFeatureSet {
            bits: 1 << 3,
            mask: 1 << 3 | Self::INCARNATION_1,
        };
        pub const SUBSCRIBE2: CephFeatureSet = CephFeatureSet {
            bits: 1 << 4,
            mask: 1 << 4 | Self::INCARNATION_1,
        };
        pub const MONNAMES: CephFeatureSet = CephFeatureSet {
            bits: 1 << 5,
            mask: 1 << 5 | Self::INCARNATION_1,
        };
        pub const RECONNECT_SEQ: CephFeatureSet = CephFeatureSet {
            bits: 1 << 6,
            mask: 1 << 6 | Self::INCARNATION_1,
        };
        pub const DIRLAYOUTHASH: CephFeatureSet = CephFeatureSet {
            bits: 1 << 7,
            mask: 1 << 7 | Self::INCARNATION_1,
        };
        pub const OBJECTLOCATOR: CephFeatureSet = CephFeatureSet {
            bits: 1 << 8,
            mask: 1 << 8 | Self::INCARNATION_1,
        };
        pub const PGID64: CephFeatureSet = CephFeatureSet {
            bits: 1 << 9,
            mask: 1 << 9 | Self::INCARNATION_1,
        };
        pub const INCSUBOSDMAP: CephFeatureSet = CephFeatureSet {
            bits: 1 << 10,
            mask: 1 << 10 | Self::INCARNATION_1,
        };
        pub const PGPOOL3: CephFeatureSet = CephFeatureSet {
            bits: 1 << 11,
            mask: 1 << 11 | Self::INCARNATION_1,
        };
        pub const OSDREPLYMUX: CephFeatureSet = CephFeatureSet {
            bits: 1 << 12,
            mask: 1 << 12 | Self::INCARNATION_1,
        };
        pub const OSDENC: CephFeatureSet = CephFeatureSet {
            bits: 1 << 13,
            mask: 1 << 13 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const OMAP: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_KRAKEN: CephFeatureSet = CephFeatureSet {
            bits: 1 << 14,
            mask: 1 << 14 | Self::INCARNATION_2,
        };
        pub const MONENC: CephFeatureSet = CephFeatureSet {
            bits: 1 << 15,
            mask: 1 << 15 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const QUERY_T: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_OCTOPUS: CephFeatureSet = CephFeatureSet {
            bits: 1 << 16,
            mask: 1 << 16 | Self::INCARNATION_3,
        };
        pub const OSD_REPOP_MLCOD: CephFeatureSet = CephFeatureSet {
            bits: 1 << 16,
            mask: 1 << 16 | Self::INCARNATION_3,
        };
        #[allow(unused)]
        const INDEP_PG_MAP: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const OS_PERF_STAT_NS: CephFeatureSet = CephFeatureSet {
            bits: 1 << 17,
            mask: 1 << 17 | Self::INCARNATION_3,
        };
        pub const CRUSH_TUNABLES: CephFeatureSet = CephFeatureSet {
            bits: 1 << 18,
            mask: 1 << 18 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const CHUNKY_SCRUB: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const OSD_PGLOG_HARDLIMIT: CephFeatureSet = CephFeatureSet {
            bits: 1 << 19,
            mask: 1 << 19 | Self::INCARNATION_2,
        };
        #[allow(unused)]
        const MON_NULLROUTE: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_PACIFIC: CephFeatureSet = CephFeatureSet {
            bits: 1 << 20,
            mask: 1 << 20 | Self::INCARNATION_3,
        };
        #[allow(unused)]
        const MON_GV: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_LUMINOUS: CephFeatureSet = CephFeatureSet {
            bits: 1 << 21,
            mask: 1 << 21 | Self::INCARNATION_2,
        };
        pub const RESEND_ON_SPLIT: CephFeatureSet = CephFeatureSet {
            bits: 1 << 21,
            mask: 1 << 21 | Self::INCARNATION_2,
        };
        pub const RADOS_BACKOFF: CephFeatureSet = CephFeatureSet {
            bits: 1 << 21,
            mask: 1 << 21 | Self::INCARNATION_2,
        };
        pub const OSDMAP_PG_UPMAP: CephFeatureSet = CephFeatureSet {
            bits: 1 << 21,
            mask: 1 << 21 | Self::INCARNATION_2,
        };
        pub const CRUSH_CHOOSE_ARGS: CephFeatureSet = CephFeatureSet {
            bits: 1 << 21,
            mask: 1 << 21 | Self::INCARNATION_2,
        };
        #[allow(unused)]
        const BACKFILL_RESERVATION: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const OSD_FIXED_COLLECTION_LIST: CephFeatureSet = CephFeatureSet {
            bits: 1 << 22,
            mask: 1 << 22 | Self::INCARNATION_2,
        };
        pub const MSG_AUTH: CephFeatureSet = CephFeatureSet {
            bits: 1 << 23,
            mask: 1 << 23 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const RECOVERY_RESERVATION: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const RECOVERY_RESERVATION_2: CephFeatureSet = CephFeatureSet {
            bits: 1 << 24,
            mask: 1 << 24 | Self::INCARNATION_2,
        };
        pub const CRUSH_TUNABLES2: CephFeatureSet = CephFeatureSet {
            bits: 1 << 25,
            mask: 1 << 25 | Self::INCARNATION_1,
        };
        pub const CREATEPOOLID: CephFeatureSet = CephFeatureSet {
            bits: 1 << 26,
            mask: 1 << 26 | Self::INCARNATION_1,
        };
        pub const REPLY_CREATE_INODE: CephFeatureSet = CephFeatureSet {
            bits: 1 << 27,
            mask: 1 << 27 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const OSD_HBMSGS: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_MIMIC: CephFeatureSet = CephFeatureSet {
            bits: 1 << 28,
            mask: 1 << 28 | Self::INCARNATION_2,
        };
        pub const MDSENC: CephFeatureSet = CephFeatureSet {
            bits: 1 << 29,
            mask: 1 << 29 | Self::INCARNATION_1,
        };
        pub const OSDHASHPSPOOL: CephFeatureSet = CephFeatureSet {
            bits: 1 << 30,
            mask: 1 << 30 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const MON_SINGLE_PAXOS: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_REEF: CephFeatureSet = CephFeatureSet {
            bits: 1 << 31,
            mask: 1 << 31 | Self::INCARNATION_3,
        };
        #[allow(unused)]
        const OSD_SNAPMAPPER: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const STRETCH_MODE: CephFeatureSet = CephFeatureSet {
            bits: 1 << 32,
            mask: 1 << 32 | Self::INCARNATION_3,
        };
        #[allow(unused)]
        const MON_SCRUB: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_QUINCY: CephFeatureSet = CephFeatureSet {
            bits: 1 << 33,
            mask: 1 << 33 | Self::INCARNATION_3,
        };
        #[allow(unused)]
        const OSD_PACKED_RECOVERY: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const RANGE_BLOCKLIST: CephFeatureSet = CephFeatureSet {
            bits: 1 << 34,
            mask: 1 << 34 | Self::INCARNATION_3,
        };
        pub const OSD_CACHEPOOL: CephFeatureSet = CephFeatureSet {
            bits: 1 << 35,
            mask: 1 << 35 | Self::INCARNATION_1,
        };
        pub const CRUSH_V2: CephFeatureSet = CephFeatureSet {
            bits: 1 << 36,
            mask: 1 << 36 | Self::INCARNATION_1,
        };
        pub const EXPORT_PEER: CephFeatureSet = CephFeatureSet {
            bits: 1 << 37,
            mask: 1 << 37 | Self::INCARNATION_1,
        };
        pub const CRUSH_MSR: CephFeatureSet = CephFeatureSet {
            bits: 1 << 38,
            mask: 1 << 38 | Self::INCARNATION_2,
        };
        pub const OSDMAP_ENC: CephFeatureSet = CephFeatureSet {
            bits: 1 << 39,
            mask: 1 << 39 | Self::INCARNATION_1,
        };
        pub const MDS_INLINE_DATA: CephFeatureSet = CephFeatureSet {
            bits: 1 << 40,
            mask: 1 << 40 | Self::INCARNATION_1,
        };
        pub const CRUSH_TUNABLES3: CephFeatureSet = CephFeatureSet {
            bits: 1 << 41,
            mask: 1 << 41 | Self::INCARNATION_1,
        };
        pub const OSD_PRIMARY_AFFINITY: CephFeatureSet = CephFeatureSet {
            bits: 1 << 41,
            mask: 1 << 41 | Self::INCARNATION_1,
        };
        pub const MSGR_KEEPALIVE2: CephFeatureSet = CephFeatureSet {
            bits: 1 << 42,
            mask: 1 << 42 | Self::INCARNATION_1,
        };
        pub const OSD_POOLRESEND: CephFeatureSet = CephFeatureSet {
            bits: 1 << 43,
            mask: 1 << 43 | Self::INCARNATION_1,
        };
        pub const NVMEOFHA: CephFeatureSet = CephFeatureSet {
            bits: 1 << 44,
            mask: 1 << 44 | Self::INCARNATION_2,
        };
        #[allow(unused)]
        const OSD_SET_ALLOC_HINT: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const NVMEOFHAMAP: CephFeatureSet = CephFeatureSet {
            bits: 1 << 45,
            mask: 1 << 45 | Self::INCARNATION_2,
        };
        pub const OSD_FADVISE_FLAGS: CephFeatureSet = CephFeatureSet {
            bits: 1 << 46,
            mask: 1 << 46 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const OSD_REPOP: CephFeatureSet = CephFeatureSet::EMPTY;
        #[allow(unused)]
        const OSD_OBJECT_DIGEST: CephFeatureSet = CephFeatureSet::EMPTY;
        #[allow(unused)]
        const OSD_TRANSACTION_MAY_LAYOUT: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const MDS_QUOTA: CephFeatureSet = CephFeatureSet {
            bits: 1 << 47,
            mask: 1 << 47 | Self::INCARNATION_1,
        };
        pub const CRUSH_V4: CephFeatureSet = CephFeatureSet {
            bits: 1 << 48,
            mask: 1 << 48 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const OSD_MIN_SIZE_RECOVERY: CephFeatureSet = CephFeatureSet::EMPTY;
        #[allow(unused)]
        const OSD_PROXY_FEATURES: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_SQUID: CephFeatureSet = CephFeatureSet {
            bits: 1 << 49,
            mask: 1 << 49 | Self::INCARNATION_2,
        };
        #[allow(unused)]
        const MON_METADATA: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_TENTACLE: CephFeatureSet = CephFeatureSet {
            bits: 1 << 50,
            mask: 1 << 50 | Self::INCARNATION_2,
        };
        #[allow(unused)]
        const OSD_BITWISE_HOBJ_SORT: CephFeatureSet = CephFeatureSet::EMPTY;
        #[allow(unused)]
        const OSD_PROXY_WRITE_FEATURES: CephFeatureSet = CephFeatureSet::EMPTY;
        #[allow(unused)]
        const ERASURE_CODE_PLUGINS_V3: CephFeatureSet = CephFeatureSet::EMPTY;
        #[allow(unused)]
        const OSD_HITSET_GMT: CephFeatureSet = CephFeatureSet::EMPTY;
        #[allow(unused)]
        const HAMMER_0_94_4: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const NEW_OSDOP_ENCODING: CephFeatureSet = CephFeatureSet {
            bits: 1 << 56,
            mask: 1 << 56 | Self::INCARNATION_1,
        };
        pub const MON_STATEFUL_SUB: CephFeatureSet = CephFeatureSet {
            bits: 1 << 57,
            mask: 1 << 57 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const MON_ROUTE_OSDMAP: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const SERVER_JEWEL: CephFeatureSet = CephFeatureSet {
            bits: 1 << 57,
            mask: 1 << 57 | Self::INCARNATION_1,
        };
        pub const CRUSH_TUNABLES5: CephFeatureSet = CephFeatureSet {
            bits: 1 << 58,
            mask: 1 << 58 | Self::INCARNATION_1,
        };
        pub const NEW_OSDOPREPLY_ENCODING: CephFeatureSet = CephFeatureSet {
            bits: 1 << 58,
            mask: 1 << 58 | Self::INCARNATION_1,
        };
        pub const FS_FILE_LAYOUT_V2: CephFeatureSet = CephFeatureSet {
            bits: 1 << 58,
            mask: 1 << 58 | Self::INCARNATION_1,
        };
        pub const FS_BTIME: CephFeatureSet = CephFeatureSet {
            bits: 1 << 59,
            mask: 1 << 59 | Self::INCARNATION_1,
        };
        pub const FS_CHANGE_ATTR: CephFeatureSet = CephFeatureSet {
            bits: 1 << 59,
            mask: 1 << 59 | Self::INCARNATION_1,
        };
        pub const MSG_ADDR2: CephFeatureSet = CephFeatureSet {
            bits: 1 << 59,
            mask: 1 << 59 | Self::INCARNATION_1,
        };
        pub const OSD_RECOVERY_DELETES: CephFeatureSet = CephFeatureSet {
            bits: 1 << 60,
            mask: 1 << 60 | Self::INCARNATION_1,
        };
        pub const CEPHX_V2: CephFeatureSet = CephFeatureSet {
            bits: 1 << 61,
            mask: 1 << 61 | Self::INCARNATION_1,
        };
        pub const RESERVED: CephFeatureSet = CephFeatureSet {
            bits: 1 << 62,
            mask: 1 << 62 | Self::INCARNATION_1,
        };
        #[allow(unused)]
        const RESERVED_BROKEN: CephFeatureSet = CephFeatureSet::EMPTY;
        pub const LIST: &'static [CephFeatureSet] = &[
            CephFeatureSet::UID,
            CephFeatureSet::NOSRCADDR,
            CephFeatureSet::MONCLOCKCHECK,
            CephFeatureSet::SERVER_NAUTILUS,
            CephFeatureSet::FLOCK,
            CephFeatureSet::SUBSCRIBE2,
            CephFeatureSet::MONNAMES,
            CephFeatureSet::RECONNECT_SEQ,
            CephFeatureSet::DIRLAYOUTHASH,
            CephFeatureSet::OBJECTLOCATOR,
            CephFeatureSet::PGID64,
            CephFeatureSet::INCSUBOSDMAP,
            CephFeatureSet::PGPOOL3,
            CephFeatureSet::OSDREPLYMUX,
            CephFeatureSet::OSDENC,
            CephFeatureSet::OMAP,
            CephFeatureSet::SERVER_KRAKEN,
            CephFeatureSet::MONENC,
            CephFeatureSet::QUERY_T,
            CephFeatureSet::SERVER_OCTOPUS,
            CephFeatureSet::OSD_REPOP_MLCOD,
            CephFeatureSet::INDEP_PG_MAP,
            CephFeatureSet::OS_PERF_STAT_NS,
            CephFeatureSet::CRUSH_TUNABLES,
            CephFeatureSet::CHUNKY_SCRUB,
            CephFeatureSet::OSD_PGLOG_HARDLIMIT,
            CephFeatureSet::MON_NULLROUTE,
            CephFeatureSet::SERVER_PACIFIC,
            CephFeatureSet::MON_GV,
            CephFeatureSet::SERVER_LUMINOUS,
            CephFeatureSet::RESEND_ON_SPLIT,
            CephFeatureSet::RADOS_BACKOFF,
            CephFeatureSet::OSDMAP_PG_UPMAP,
            CephFeatureSet::CRUSH_CHOOSE_ARGS,
            CephFeatureSet::BACKFILL_RESERVATION,
            CephFeatureSet::OSD_FIXED_COLLECTION_LIST,
            CephFeatureSet::MSG_AUTH,
            CephFeatureSet::RECOVERY_RESERVATION,
            CephFeatureSet::RECOVERY_RESERVATION_2,
            CephFeatureSet::CRUSH_TUNABLES2,
            CephFeatureSet::CREATEPOOLID,
            CephFeatureSet::REPLY_CREATE_INODE,
            CephFeatureSet::OSD_HBMSGS,
            CephFeatureSet::SERVER_MIMIC,
            CephFeatureSet::MDSENC,
            CephFeatureSet::OSDHASHPSPOOL,
            CephFeatureSet::MON_SINGLE_PAXOS,
            CephFeatureSet::SERVER_REEF,
            CephFeatureSet::OSD_SNAPMAPPER,
            CephFeatureSet::STRETCH_MODE,
            CephFeatureSet::MON_SCRUB,
            CephFeatureSet::SERVER_QUINCY,
            CephFeatureSet::OSD_PACKED_RECOVERY,
            CephFeatureSet::RANGE_BLOCKLIST,
            CephFeatureSet::OSD_CACHEPOOL,
            CephFeatureSet::CRUSH_V2,
            CephFeatureSet::EXPORT_PEER,
            CephFeatureSet::CRUSH_MSR,
            CephFeatureSet::OSDMAP_ENC,
            CephFeatureSet::MDS_INLINE_DATA,
            CephFeatureSet::CRUSH_TUNABLES3,
            CephFeatureSet::OSD_PRIMARY_AFFINITY,
            CephFeatureSet::MSGR_KEEPALIVE2,
            CephFeatureSet::OSD_POOLRESEND,
            CephFeatureSet::NVMEOFHA,
            CephFeatureSet::OSD_SET_ALLOC_HINT,
            CephFeatureSet::NVMEOFHAMAP,
            CephFeatureSet::OSD_FADVISE_FLAGS,
            CephFeatureSet::OSD_REPOP,
            CephFeatureSet::OSD_OBJECT_DIGEST,
            CephFeatureSet::OSD_TRANSACTION_MAY_LAYOUT,
            CephFeatureSet::MDS_QUOTA,
            CephFeatureSet::CRUSH_V4,
            CephFeatureSet::OSD_MIN_SIZE_RECOVERY,
            CephFeatureSet::OSD_PROXY_FEATURES,
            CephFeatureSet::SERVER_SQUID,
            CephFeatureSet::MON_METADATA,
            CephFeatureSet::SERVER_TENTACLE,
            CephFeatureSet::OSD_BITWISE_HOBJ_SORT,
            CephFeatureSet::OSD_PROXY_WRITE_FEATURES,
            CephFeatureSet::ERASURE_CODE_PLUGINS_V3,
            CephFeatureSet::OSD_HITSET_GMT,
            CephFeatureSet::HAMMER_0_94_4,
            CephFeatureSet::NEW_OSDOP_ENCODING,
            CephFeatureSet::MON_STATEFUL_SUB,
            CephFeatureSet::MON_ROUTE_OSDMAP,
            CephFeatureSet::SERVER_JEWEL,
            CephFeatureSet::CRUSH_TUNABLES5,
            CephFeatureSet::NEW_OSDOPREPLY_ENCODING,
            CephFeatureSet::FS_FILE_LAYOUT_V2,
            CephFeatureSet::FS_BTIME,
            CephFeatureSet::FS_CHANGE_ATTR,
            CephFeatureSet::MSG_ADDR2,
            CephFeatureSet::OSD_RECOVERY_DELETES,
            CephFeatureSet::CEPHX_V2,
            CephFeatureSet::RESERVED,
            CephFeatureSet::RESERVED_BROKEN,
        ];
        pub const ALL: CephFeatureSet = CephFeatureSet::EMPTY
            .or(CephFeatureSet::UID)
            .or(CephFeatureSet::NOSRCADDR)
            .or(CephFeatureSet::MONCLOCKCHECK)
            .or(CephFeatureSet::SERVER_NAUTILUS)
            .or(CephFeatureSet::FLOCK)
            .or(CephFeatureSet::SUBSCRIBE2)
            .or(CephFeatureSet::MONNAMES)
            .or(CephFeatureSet::RECONNECT_SEQ)
            .or(CephFeatureSet::DIRLAYOUTHASH)
            .or(CephFeatureSet::OBJECTLOCATOR)
            .or(CephFeatureSet::PGID64)
            .or(CephFeatureSet::INCSUBOSDMAP)
            .or(CephFeatureSet::PGPOOL3)
            .or(CephFeatureSet::OSDREPLYMUX)
            .or(CephFeatureSet::OSDENC)
            .or(CephFeatureSet::OMAP)
            .or(CephFeatureSet::SERVER_KRAKEN)
            .or(CephFeatureSet::MONENC)
            .or(CephFeatureSet::QUERY_T)
            .or(CephFeatureSet::SERVER_OCTOPUS)
            .or(CephFeatureSet::OSD_REPOP_MLCOD)
            .or(CephFeatureSet::INDEP_PG_MAP)
            .or(CephFeatureSet::OS_PERF_STAT_NS)
            .or(CephFeatureSet::CRUSH_TUNABLES)
            .or(CephFeatureSet::CHUNKY_SCRUB)
            .or(CephFeatureSet::OSD_PGLOG_HARDLIMIT)
            .or(CephFeatureSet::MON_NULLROUTE)
            .or(CephFeatureSet::SERVER_PACIFIC)
            .or(CephFeatureSet::MON_GV)
            .or(CephFeatureSet::SERVER_LUMINOUS)
            .or(CephFeatureSet::RESEND_ON_SPLIT)
            .or(CephFeatureSet::RADOS_BACKOFF)
            .or(CephFeatureSet::OSDMAP_PG_UPMAP)
            .or(CephFeatureSet::CRUSH_CHOOSE_ARGS)
            .or(CephFeatureSet::BACKFILL_RESERVATION)
            .or(CephFeatureSet::OSD_FIXED_COLLECTION_LIST)
            .or(CephFeatureSet::MSG_AUTH)
            .or(CephFeatureSet::RECOVERY_RESERVATION)
            .or(CephFeatureSet::RECOVERY_RESERVATION_2)
            .or(CephFeatureSet::CRUSH_TUNABLES2)
            .or(CephFeatureSet::CREATEPOOLID)
            .or(CephFeatureSet::REPLY_CREATE_INODE)
            .or(CephFeatureSet::OSD_HBMSGS)
            .or(CephFeatureSet::SERVER_MIMIC)
            .or(CephFeatureSet::MDSENC)
            .or(CephFeatureSet::OSDHASHPSPOOL)
            .or(CephFeatureSet::MON_SINGLE_PAXOS)
            .or(CephFeatureSet::SERVER_REEF)
            .or(CephFeatureSet::OSD_SNAPMAPPER)
            .or(CephFeatureSet::STRETCH_MODE)
            .or(CephFeatureSet::MON_SCRUB)
            .or(CephFeatureSet::SERVER_QUINCY)
            .or(CephFeatureSet::OSD_PACKED_RECOVERY)
            .or(CephFeatureSet::RANGE_BLOCKLIST)
            .or(CephFeatureSet::OSD_CACHEPOOL)
            .or(CephFeatureSet::CRUSH_V2)
            .or(CephFeatureSet::EXPORT_PEER)
            .or(CephFeatureSet::CRUSH_MSR)
            .or(CephFeatureSet::OSDMAP_ENC)
            .or(CephFeatureSet::MDS_INLINE_DATA)
            .or(CephFeatureSet::CRUSH_TUNABLES3)
            .or(CephFeatureSet::OSD_PRIMARY_AFFINITY)
            .or(CephFeatureSet::MSGR_KEEPALIVE2)
            .or(CephFeatureSet::OSD_POOLRESEND)
            .or(CephFeatureSet::NVMEOFHA)
            .or(CephFeatureSet::OSD_SET_ALLOC_HINT)
            .or(CephFeatureSet::NVMEOFHAMAP)
            .or(CephFeatureSet::OSD_FADVISE_FLAGS)
            .or(CephFeatureSet::OSD_REPOP)
            .or(CephFeatureSet::OSD_OBJECT_DIGEST)
            .or(CephFeatureSet::OSD_TRANSACTION_MAY_LAYOUT)
            .or(CephFeatureSet::MDS_QUOTA)
            .or(CephFeatureSet::CRUSH_V4)
            .or(CephFeatureSet::OSD_MIN_SIZE_RECOVERY)
            .or(CephFeatureSet::OSD_PROXY_FEATURES)
            .or(CephFeatureSet::SERVER_SQUID)
            .or(CephFeatureSet::MON_METADATA)
            .or(CephFeatureSet::SERVER_TENTACLE)
            .or(CephFeatureSet::OSD_BITWISE_HOBJ_SORT)
            .or(CephFeatureSet::OSD_PROXY_WRITE_FEATURES)
            .or(CephFeatureSet::ERASURE_CODE_PLUGINS_V3)
            .or(CephFeatureSet::OSD_HITSET_GMT)
            .or(CephFeatureSet::HAMMER_0_94_4)
            .or(CephFeatureSet::NEW_OSDOP_ENCODING)
            .or(CephFeatureSet::MON_STATEFUL_SUB)
            .or(CephFeatureSet::MON_ROUTE_OSDMAP)
            .or(CephFeatureSet::SERVER_JEWEL)
            .or(CephFeatureSet::CRUSH_TUNABLES5)
            .or(CephFeatureSet::NEW_OSDOPREPLY_ENCODING)
            .or(CephFeatureSet::FS_FILE_LAYOUT_V2)
            .or(CephFeatureSet::FS_BTIME)
            .or(CephFeatureSet::FS_CHANGE_ATTR)
            .or(CephFeatureSet::MSG_ADDR2)
            .or(CephFeatureSet::OSD_RECOVERY_DELETES)
            .or(CephFeatureSet::CEPHX_V2)
            .or(CephFeatureSet::RESERVED)
            .or(CephFeatureSet::RESERVED_BROKEN);
    }
    impl core::fmt::Display for CephFeatureSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut output_any = false;
            if self.has(&CephFeatureSet::UID) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "UID"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "UID"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::NOSRCADDR) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "NOSRCADDR"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "NOSRCADDR"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MONCLOCKCHECK) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MONCLOCKCHECK"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MONCLOCKCHECK"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_NAUTILUS) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_NAUTILUS"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_NAUTILUS"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::FLOCK) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "FLOCK"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "FLOCK"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SUBSCRIBE2) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SUBSCRIBE2"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SUBSCRIBE2"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MONNAMES) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MONNAMES"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MONNAMES"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::RECONNECT_SEQ) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "RECONNECT_SEQ"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "RECONNECT_SEQ"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::DIRLAYOUTHASH) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "DIRLAYOUTHASH"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "DIRLAYOUTHASH"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OBJECTLOCATOR) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OBJECTLOCATOR"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OBJECTLOCATOR"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::PGID64) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "PGID64"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "PGID64"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::INCSUBOSDMAP) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "INCSUBOSDMAP"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "INCSUBOSDMAP"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::PGPOOL3) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "PGPOOL3"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "PGPOOL3"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSDREPLYMUX) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSDREPLYMUX"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSDREPLYMUX"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSDENC) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSDENC"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSDENC"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OMAP) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OMAP"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OMAP"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_KRAKEN) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_KRAKEN"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_KRAKEN"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MONENC) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MONENC"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MONENC"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::QUERY_T) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "QUERY_T"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "QUERY_T"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_OCTOPUS) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_OCTOPUS"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_OCTOPUS"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_REPOP_MLCOD) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_REPOP_MLCOD"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_REPOP_MLCOD"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::INDEP_PG_MAP) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "INDEP_PG_MAP"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "INDEP_PG_MAP"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OS_PERF_STAT_NS) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OS_PERF_STAT_NS"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OS_PERF_STAT_NS"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CRUSH_TUNABLES) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CRUSH_TUNABLES"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CRUSH_TUNABLES"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CHUNKY_SCRUB) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CHUNKY_SCRUB"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CHUNKY_SCRUB"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_PGLOG_HARDLIMIT) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_PGLOG_HARDLIMIT"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_PGLOG_HARDLIMIT"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MON_NULLROUTE) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MON_NULLROUTE"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MON_NULLROUTE"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_PACIFIC) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_PACIFIC"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_PACIFIC"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MON_GV) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MON_GV"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MON_GV"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_LUMINOUS) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_LUMINOUS"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_LUMINOUS"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::RESEND_ON_SPLIT) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "RESEND_ON_SPLIT"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "RESEND_ON_SPLIT"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::RADOS_BACKOFF) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "RADOS_BACKOFF"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "RADOS_BACKOFF"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSDMAP_PG_UPMAP) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSDMAP_PG_UPMAP"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSDMAP_PG_UPMAP"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CRUSH_CHOOSE_ARGS) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CRUSH_CHOOSE_ARGS"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CRUSH_CHOOSE_ARGS"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::BACKFILL_RESERVATION) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "BACKFILL_RESERVATION"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "BACKFILL_RESERVATION"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_FIXED_COLLECTION_LIST) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_FIXED_COLLECTION_LIST"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_FIXED_COLLECTION_LIST"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MSG_AUTH) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MSG_AUTH"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MSG_AUTH"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::RECOVERY_RESERVATION) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "RECOVERY_RESERVATION"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "RECOVERY_RESERVATION"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::RECOVERY_RESERVATION_2) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "RECOVERY_RESERVATION_2"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "RECOVERY_RESERVATION_2"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CRUSH_TUNABLES2) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CRUSH_TUNABLES2"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CRUSH_TUNABLES2"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CREATEPOOLID) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CREATEPOOLID"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CREATEPOOLID"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::REPLY_CREATE_INODE) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "REPLY_CREATE_INODE"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "REPLY_CREATE_INODE"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_HBMSGS) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_HBMSGS"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_HBMSGS"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_MIMIC) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_MIMIC"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_MIMIC"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MDSENC) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MDSENC"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MDSENC"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSDHASHPSPOOL) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSDHASHPSPOOL"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSDHASHPSPOOL"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MON_SINGLE_PAXOS) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MON_SINGLE_PAXOS"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MON_SINGLE_PAXOS"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_REEF) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_REEF"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_REEF"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_SNAPMAPPER) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_SNAPMAPPER"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_SNAPMAPPER"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::STRETCH_MODE) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "STRETCH_MODE"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "STRETCH_MODE"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MON_SCRUB) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MON_SCRUB"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MON_SCRUB"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_QUINCY) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_QUINCY"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_QUINCY"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_PACKED_RECOVERY) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_PACKED_RECOVERY"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_PACKED_RECOVERY"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::RANGE_BLOCKLIST) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "RANGE_BLOCKLIST"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "RANGE_BLOCKLIST"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_CACHEPOOL) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_CACHEPOOL"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_CACHEPOOL"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CRUSH_V2) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CRUSH_V2"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CRUSH_V2"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::EXPORT_PEER) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "EXPORT_PEER"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "EXPORT_PEER"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CRUSH_MSR) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CRUSH_MSR"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CRUSH_MSR"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSDMAP_ENC) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSDMAP_ENC"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSDMAP_ENC"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MDS_INLINE_DATA) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MDS_INLINE_DATA"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MDS_INLINE_DATA"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CRUSH_TUNABLES3) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CRUSH_TUNABLES3"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CRUSH_TUNABLES3"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_PRIMARY_AFFINITY) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_PRIMARY_AFFINITY"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_PRIMARY_AFFINITY"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MSGR_KEEPALIVE2) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MSGR_KEEPALIVE2"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MSGR_KEEPALIVE2"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_POOLRESEND) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_POOLRESEND"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_POOLRESEND"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::NVMEOFHA) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "NVMEOFHA"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "NVMEOFHA"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_SET_ALLOC_HINT) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_SET_ALLOC_HINT"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_SET_ALLOC_HINT"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::NVMEOFHAMAP) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "NVMEOFHAMAP"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "NVMEOFHAMAP"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_FADVISE_FLAGS) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_FADVISE_FLAGS"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_FADVISE_FLAGS"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_REPOP) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_REPOP"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_REPOP"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_OBJECT_DIGEST) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_OBJECT_DIGEST"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_OBJECT_DIGEST"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_TRANSACTION_MAY_LAYOUT) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_TRANSACTION_MAY_LAYOUT"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_TRANSACTION_MAY_LAYOUT"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MDS_QUOTA) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MDS_QUOTA"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MDS_QUOTA"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CRUSH_V4) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CRUSH_V4"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CRUSH_V4"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_MIN_SIZE_RECOVERY) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_MIN_SIZE_RECOVERY"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_MIN_SIZE_RECOVERY"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_PROXY_FEATURES) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_PROXY_FEATURES"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_PROXY_FEATURES"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_SQUID) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_SQUID"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_SQUID"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MON_METADATA) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MON_METADATA"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MON_METADATA"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_TENTACLE) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_TENTACLE"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_TENTACLE"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_BITWISE_HOBJ_SORT) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_BITWISE_HOBJ_SORT"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_BITWISE_HOBJ_SORT"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_PROXY_WRITE_FEATURES) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_PROXY_WRITE_FEATURES"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_PROXY_WRITE_FEATURES"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::ERASURE_CODE_PLUGINS_V3) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "ERASURE_CODE_PLUGINS_V3"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "ERASURE_CODE_PLUGINS_V3"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_HITSET_GMT) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_HITSET_GMT"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_HITSET_GMT"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::HAMMER_0_94_4) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "HAMMER_0_94_4"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "HAMMER_0_94_4"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::NEW_OSDOP_ENCODING) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "NEW_OSDOP_ENCODING"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "NEW_OSDOP_ENCODING"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MON_STATEFUL_SUB) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MON_STATEFUL_SUB"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MON_STATEFUL_SUB"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MON_ROUTE_OSDMAP) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MON_ROUTE_OSDMAP"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MON_ROUTE_OSDMAP"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::SERVER_JEWEL) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "SERVER_JEWEL"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "SERVER_JEWEL"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CRUSH_TUNABLES5) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CRUSH_TUNABLES5"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CRUSH_TUNABLES5"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::NEW_OSDOPREPLY_ENCODING) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "NEW_OSDOPREPLY_ENCODING"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "NEW_OSDOPREPLY_ENCODING"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::FS_FILE_LAYOUT_V2) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "FS_FILE_LAYOUT_V2"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "FS_FILE_LAYOUT_V2"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::FS_BTIME) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "FS_BTIME"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "FS_BTIME"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::FS_CHANGE_ATTR) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "FS_CHANGE_ATTR"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "FS_CHANGE_ATTR"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::MSG_ADDR2) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "MSG_ADDR2"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "MSG_ADDR2"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::OSD_RECOVERY_DELETES) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "OSD_RECOVERY_DELETES"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "OSD_RECOVERY_DELETES"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::CEPHX_V2) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "CEPHX_V2"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "CEPHX_V2"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::RESERVED) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "RESERVED"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "RESERVED"))?;
                }
                output_any = true;
            }
            if self.has(&CephFeatureSet::RESERVED_BROKEN) {
                if !output_any {
                    f.write_fmt(format_args!("{0}", "RESERVED_BROKEN"))?;
                } else {
                    f.write_fmt(format_args!("| {0}", "RESERVED_BROKEN"))?;
                }
                output_any = true;
            }
            Ok(())
        }
    }
    impl CephFeatureSet {
        const INCARNATION_1: u64 = 0;
        const INCARNATION_2: u64 = 1 << 57;
        const INCARNATION_3: u64 = 1 << 57 | 1 << 28;
        pub const EMPTY: CephFeatureSet = CephFeatureSet { bits: 0, mask: 0 };
        pub fn get(&self) -> u64 {
            self.bits
        }
        pub const fn or(self, rhs: Self) -> Self {
            Self {
                bits: self.bits | rhs.bits,
                mask: self.mask | rhs.mask,
            }
        }
        pub const fn and(self, rhs: Self) -> Self {
            Self {
                bits: self.bits & rhs.bits,
                mask: self.mask & rhs.mask,
            }
        }
        pub fn has(&self, features: &CephFeatureSet) -> bool {
            self.bits & features.mask == features.mask
        }
    }
    impl core::ops::BitOr for CephFeatureSet {
        type Output = CephFeatureSet;
        fn bitor(self, rhs: Self) -> Self::Output {
            self.or(rhs)
        }
    }
    impl core::ops::BitOrAssign for CephFeatureSet {
        fn bitor_assign(&mut self, rhs: Self) {
            *self = *self | rhs;
        }
    }
    impl core::ops::BitAnd for CephFeatureSet {
        type Output = CephFeatureSet;
        fn bitand(self, rhs: Self) -> Self::Output {
            self.and(rhs)
        }
    }
    impl core::ops::BitAndAssign for CephFeatureSet {
        fn bitand_assign(&mut self, rhs: Self) {
            *self = *self & rhs;
        }
    }
}
mod connection {
    use crate::{
        Encode,
        frame::{Frame, Preamble, Tag},
        messages::{
            Banner, ClientIdent, Hello, IdentMissingFeatures, Keepalive, MsgrFeatures,
            auth::{AuthDone, AuthRequest, AuthSignature},
        },
    };
    enum State {
        Inactive,
        Active,
    }
    impl State {
        fn is_active(&self) -> bool {
            #[allow(non_exhaustive_omitted_patterns)]
            match self {
                Self::Active => true,
                _ => false,
            }
        }
        fn is_inactive(&self) -> bool {
            #[allow(non_exhaustive_omitted_patterns)]
            match self {
                Self::Inactive => true,
                _ => false,
            }
        }
    }
    pub struct Connection {
        state: State,
        buffer: Vec<u8>,
    }
    impl Connection {
        pub fn new() -> Self {
            Self {
                state: State::Inactive,
                buffer: Vec::new(),
            }
        }
        pub fn banner(&self) -> Banner {
            Banner::new(MsgrFeatures::empty(), MsgrFeatures::empty())
        }
        pub fn preamble_len(&self) -> usize {
            crate::frame::Preamble::SERIALIZED_SIZE
        }
        pub fn recv_banner(&mut self, banner: &Banner) -> Result<(), String> {
            if !self.state.is_inactive() {
                ::core::panicking::panic("assertion failed: self.state.is_inactive()")
            };
            if banner.required().compression() {
                return Err("Peer requires compression, which we do not support.".into());
            }
            if banner.required().revision_21() {
                return Err("Peer requires msgr revision 2.1, which we do not support".into());
            }
            self.state = State::Active;
            Ok(())
        }
        pub fn recv_preamble(&mut self, preamble_data: &[u8]) -> Result<Preamble, String> {
            if !self.state.is_active() {
                ::core::panicking::panic("assertion failed: self.state.is_active()")
            };
            if preamble_data.len() != self.preamble_len() {
                return Err(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!(
                        "Expected {0} bytes of preamble data, got {1}",
                        self.preamble_len(),
                        preamble_data.len()
                    ))
                }));
            }
            let preamble = Preamble::parse(preamble_data)?;
            Ok(preamble)
        }
        pub fn recv(&mut self, frame: Frame) -> Result<Message, String> {
            if !self.state.is_active() {
                ::core::panicking::panic("assertion failed: self.state.is_active()")
            };
            if !(frame.segments().len() == 1) {
                {
                    ::core::panicking::panic_fmt(format_args!(
                        "Multi-segment frames not supported yet."
                    ));
                }
            };
            Ok(Message::parse(frame.tag(), frame.segments()[0])?)
        }
        pub fn send<'a, T>(&'a mut self, message: T) -> Frame<'a>
        where
            T: Into<Message>,
        {
            self.send_msg(&message.into())
        }
        pub fn send_msg<'a>(&'a mut self, message: &Message) -> Frame<'a> {
            if !self.state.is_active() {
                ::core::panicking::panic("assertion failed: self.state.is_active()")
            };
            self.buffer.clear();
            message.write_to(&mut self.buffer);
            Frame::new(message.tag(), &[&self.buffer]).unwrap()
        }
    }
    pub enum Message {
        Hello(Hello),
        ClientIdent(ClientIdent),
        AuthRequest(AuthRequest),
        AuthDone(AuthDone),
        AuthSignature(AuthSignature),
        Keepalive(Keepalive),
        IdentMissingFeatures(IdentMissingFeatures),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Message {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Message::Hello(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Hello", &__self_0)
                }
                Message::ClientIdent(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ClientIdent", &__self_0)
                }
                Message::AuthRequest(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "AuthRequest", &__self_0)
                }
                Message::AuthDone(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "AuthDone", &__self_0)
                }
                Message::AuthSignature(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "AuthSignature", &__self_0)
                }
                Message::Keepalive(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Keepalive", &__self_0)
                }
                Message::IdentMissingFeatures(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IdentMissingFeatures",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Message {
        #[inline]
        fn clone(&self) -> Message {
            match self {
                Message::Hello(__self_0) => Message::Hello(::core::clone::Clone::clone(__self_0)),
                Message::ClientIdent(__self_0) => {
                    Message::ClientIdent(::core::clone::Clone::clone(__self_0))
                }
                Message::AuthRequest(__self_0) => {
                    Message::AuthRequest(::core::clone::Clone::clone(__self_0))
                }
                Message::AuthDone(__self_0) => {
                    Message::AuthDone(::core::clone::Clone::clone(__self_0))
                }
                Message::AuthSignature(__self_0) => {
                    Message::AuthSignature(::core::clone::Clone::clone(__self_0))
                }
                Message::Keepalive(__self_0) => {
                    Message::Keepalive(::core::clone::Clone::clone(__self_0))
                }
                Message::IdentMissingFeatures(__self_0) => {
                    Message::IdentMissingFeatures(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl From<Hello> for Message {
        fn from(value: Hello) -> Self {
            Self::Hello(value)
        }
    }
    impl From<ClientIdent> for Message {
        fn from(value: ClientIdent) -> Self {
            Self::ClientIdent(value)
        }
    }
    impl From<AuthRequest> for Message {
        fn from(value: AuthRequest) -> Self {
            Self::AuthRequest(value)
        }
    }
    impl From<AuthSignature> for Message {
        fn from(value: AuthSignature) -> Self {
            Self::AuthSignature(value)
        }
    }
    impl From<Keepalive> for Message {
        fn from(value: Keepalive) -> Self {
            Self::Keepalive(value)
        }
    }
    impl From<IdentMissingFeatures> for Message {
        fn from(value: IdentMissingFeatures) -> Self {
            Self::IdentMissingFeatures(value)
        }
    }
    impl Message {
        pub fn tag(&self) -> Tag {
            match self {
                Message::Hello(_) => Tag::Hello,
                Message::ClientIdent(_) => Tag::ClientIdent,
                Message::AuthRequest(_) => Tag::AuthRequest,
                Message::Keepalive(_) => Tag::Keepalive2,
                Message::AuthDone(_) => Tag::AuthDone,
                Message::AuthSignature(_) => Tag::AuthSignature,
                Message::IdentMissingFeatures(_) => Tag::IdentMissingFeatures,
            }
        }
        pub fn write_to(&self, buffer: &mut Vec<u8>) {
            match self {
                Message::Hello(hello) => hello.encode(buffer),
                Message::ClientIdent(client_ident) => client_ident.encode(buffer),
                Message::AuthRequest(auth_request) => auth_request.encode(buffer),
                Message::Keepalive(keepalive) => keepalive.encode(buffer),
                Message::AuthDone(_) => ::core::panicking::panic("not yet implemented"),
                Message::AuthSignature(signature) => signature.encode(buffer),
                Message::IdentMissingFeatures(ident_missing_features) => {
                    ident_missing_features.encode(buffer)
                }
            }
        }
        pub fn parse(tag: Tag, data: &[u8]) -> Result<Self, String> {
            match tag {
                Tag::Hello => Ok(Self::Hello(Hello::parse(&data)?)),
                Tag::ClientIdent => Ok(Self::ClientIdent(ClientIdent::parse(data)?)),
                Tag::AuthDone => Ok(Self::AuthDone(AuthDone::parse(data)?)),
                Tag::AuthSignature => Ok(Self::AuthSignature(AuthSignature::parse(data)?)),
                Tag::IdentMissingFeatures => Ok(Self::IdentMissingFeatures(
                    IdentMissingFeatures::parse(data)
                        .ok_or("Incorrect amount of data for ident missing features")?,
                )),
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "not yet implemented: {0}",
                        format_args!("Unsupported tag {0:?}", tag)
                    ));
                }
            }
        }
    }
}
mod encode {
    pub trait Encode {
        fn encode(&self, buffer: &mut Vec<u8>);
    }
    fn encode_len(v: usize, buffer: &mut Vec<u8>) {
        let len = u32::try_from(v).expect("Slice length does not fit into u32");
        len.encode(buffer);
    }
    impl<T> Encode for &'_ T
    where
        T: Encode,
    {
        fn encode(&self, buffer: &mut Vec<u8>) {
            (*self).encode(buffer);
        }
    }
    impl Encode for [u8] {
        fn encode(&self, buffer: &mut Vec<u8>) {
            encode_len(self.len(), buffer);
            buffer.extend_from_slice(self);
        }
    }
    impl<const N: usize> Encode for [u8; N] {
        fn encode(&self, buffer: &mut Vec<u8>) {
            buffer.extend_from_slice(self.as_slice());
        }
    }
    impl<T> Encode for [T]
    where
        T: Encode,
    {
        fn encode(&self, buffer: &mut Vec<u8>) {
            encode_len(self.len(), buffer);
            for item in self.iter() {
                item.encode(buffer)
            }
        }
    }
    impl<const N: usize, T> Encode for [T; N]
    where
        T: Encode,
    {
        fn encode(&self, buffer: &mut Vec<u8>) {
            for item in self.iter() {
                item.encode(buffer);
            }
        }
    }
    impl Encode for u16 {
        fn encode(&self, buffer: &mut Vec<u8>) {
            buffer.extend_from_slice(&self.to_le_bytes());
        }
    }
    impl Encode for u32 {
        fn encode(&self, buffer: &mut Vec<u8>) {
            buffer.extend_from_slice(&self.to_le_bytes());
        }
    }
    impl Encode for u64 {
        fn encode(&self, buffer: &mut Vec<u8>) {
            buffer.extend_from_slice(&self.to_le_bytes());
        }
    }
    impl Encode for i8 {
        fn encode(&self, buffer: &mut Vec<u8>) {
            buffer.extend_from_slice(&self.to_le_bytes());
        }
    }
    impl Encode for i16 {
        fn encode(&self, buffer: &mut Vec<u8>) {
            buffer.extend_from_slice(&self.to_le_bytes());
        }
    }
    impl Encode for i32 {
        fn encode(&self, buffer: &mut Vec<u8>) {
            buffer.extend_from_slice(&self.to_le_bytes());
        }
    }
    impl Encode for i64 {
        fn encode(&self, buffer: &mut Vec<u8>) {
            buffer.extend_from_slice(&self.to_le_bytes());
        }
    }
}
mod entity_address {
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
    use nix::libc::{AF_INET, AF_INET6};
    use crate::Encode;
    pub struct EntityAddress {
        pub ty: EntityAddressType,
        pub nonce: u32,
        pub address: Option<SocketAddr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EntityAddress {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "EntityAddress",
                "ty",
                &self.ty,
                "nonce",
                &self.nonce,
                "address",
                &&self.address,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EntityAddress {
        #[inline]
        fn clone(&self) -> EntityAddress {
            EntityAddress {
                ty: ::core::clone::Clone::clone(&self.ty),
                nonce: ::core::clone::Clone::clone(&self.nonce),
                address: ::core::clone::Clone::clone(&self.address),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EntityAddress {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EntityAddress {
        #[inline]
        fn eq(&self, other: &EntityAddress) -> bool {
            self.nonce == other.nonce && self.ty == other.ty && self.address == other.address
        }
    }
    impl Encode for EntityAddress {
        fn encode(&self, buffer: &mut Vec<u8>) {
            let address_len = self
                .address
                .map(|v| {
                    let addr_len = match v {
                        SocketAddr::V4(_) => 6,
                        SocketAddr::V6(_) => 26,
                    };
                    2 + addr_len
                })
                .unwrap_or(0) as u32;
            let len = 3 + 4 + 4 + 4 + 4 + address_len;
            [1u8, 1, 1].encode(buffer);
            let data_len = len - 3 - 4;
            data_len.encode(buffer);
            (self.ty as u32).encode(buffer);
            self.nonce.encode(buffer);
            address_len.encode(buffer);
            match self.address {
                Some(SocketAddr::V4(v4_addr)) => {
                    (AF_INET as u16).encode(buffer);
                    buffer.extend_from_slice(&v4_addr.port().to_be_bytes());
                    v4_addr.ip().octets().encode(buffer);
                }
                Some(SocketAddr::V6(v6_addr)) => {
                    (AF_INET6 as u16).encode(buffer);
                    buffer.extend_from_slice(&v6_addr.port().to_be_bytes());
                    v6_addr.flowinfo().encode(buffer);
                    v6_addr.ip().octets().encode(buffer);
                    v6_addr.scope_id().encode(buffer);
                }
                None => {}
            };
        }
    }
    impl EntityAddress {
        pub fn parse(data: &[u8]) -> Result<(usize, Self), String> {
            let mut used = 1;
            let address_version = data[0];
            match (&address_version, &1) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            used += 1;
            let encoding_version = data[1];
            match (&encoding_version, &1) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            used += 1;
            let encoding_compat = data[2];
            match (&encoding_compat, &1) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            let len = u32::from_le_bytes(data[3..7].try_into().unwrap());
            if !(data[7..].len() >= len as _) {
                ::core::panicking::panic("assertion failed: data[7..].len() >= len as _")
            };
            used += 4 + len;
            let ty = u32::from_le_bytes(data[7..11].try_into().unwrap());
            let ty = EntityAddressType::try_from(ty).map_err(|_| {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("Unknown entity type {0}", ty))
                })
            })?;
            let nonce = u32::from_le_bytes(data[11..15].try_into().unwrap());
            let address_len = u32::from_le_bytes(data[15..19].try_into().unwrap()) as usize;
            let address = if address_len != 0 {
                let family = u16::from_le_bytes(data[19..21].try_into().unwrap());
                let data = &data[21..21 + (address_len - 2)];
                if family as i32 == AF_INET {
                    let port = u16::from_be_bytes(data[..2].try_into().unwrap());
                    let address = Ipv4Addr::new(data[2], data[3], data[4], data[5]);
                    Some(SocketAddr::V4(SocketAddrV4::new(address, port)))
                } else if family as i32 == AF_INET6 {
                    let port = u16::from_be_bytes(data[..2].try_into().unwrap());
                    let flowinfo = u32::from_le_bytes(data[2..6].try_into().unwrap());
                    let address = Ipv6Addr::from_octets(data[6..22].try_into().unwrap());
                    let scope_id = u32::from_le_bytes(data[22..26].try_into().unwrap());
                    Some(SocketAddr::V6(SocketAddrV6::new(
                        address, port, flowinfo, scope_id,
                    )))
                } else {
                    return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("Unknown address family {0}", family))
                    }));
                }
            } else {
                None
            };
            Ok((used as _, Self { nonce, ty, address }))
        }
    }
    /// The type of entity that we are talking
    /// to (at the communication level).
    #[repr(u32)]
    pub enum EntityAddressType {
        None = 0,
        Legacy = 1,
        Msgr2 = 2,
        Any = 3,
        Cidr = 4,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EntityAddressType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EntityAddressType::None => "None",
                    EntityAddressType::Legacy => "Legacy",
                    EntityAddressType::Msgr2 => "Msgr2",
                    EntityAddressType::Any => "Any",
                    EntityAddressType::Cidr => "Cidr",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EntityAddressType {
        #[inline]
        fn clone(&self) -> EntityAddressType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EntityAddressType {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EntityAddressType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EntityAddressType {
        #[inline]
        fn eq(&self, other: &EntityAddressType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EntityAddressType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl TryFrom<u32> for EntityAddressType {
        type Error = ();
        fn try_from(value: u32) -> Result<Self, Self::Error> {
            let res = match value {
                0 => Self::None,
                1 => Self::Legacy,
                2 => Self::Msgr2,
                3 => Self::Any,
                4 => Self::Cidr,
                _ => return Err(()),
            };
            Ok(res)
        }
    }
    impl TryFrom<u8> for EntityAddressType {
        type Error = ();
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            Self::try_from(value as u32)
        }
    }
}
mod entity_name {
    use crate::{Encode, EntityType};
    pub struct EntityName {
        pub ty: EntityType,
        pub name: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EntityName {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "EntityName",
                "ty",
                &self.ty,
                "name",
                &&self.name,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EntityName {
        #[inline]
        fn clone(&self) -> EntityName {
            EntityName {
                ty: ::core::clone::Clone::clone(&self.ty),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    impl Encode for EntityName {
        fn encode(&self, buffer: &mut Vec<u8>) {
            (u8::from(self.ty) as u32).encode(buffer);
            self.name.as_bytes().encode(buffer);
        }
    }
}
mod entity_type {
    /// The type of entity we are talking to (MON, MDS, OSD, CLIENT or MGR).
    pub enum EntityType {
        Mon,
        Mds,
        Osd,
        Client,
        Mgr,
        Auth,
        Any,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EntityType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EntityType::Mon => "Mon",
                    EntityType::Mds => "Mds",
                    EntityType::Osd => "Osd",
                    EntityType::Client => "Client",
                    EntityType::Mgr => "Mgr",
                    EntityType::Auth => "Auth",
                    EntityType::Any => "Any",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EntityType {
        #[inline]
        fn clone(&self) -> EntityType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EntityType {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EntityType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EntityType {
        #[inline]
        fn eq(&self, other: &EntityType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EntityType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl From<EntityType> for u8 {
        fn from(value: EntityType) -> Self {
            match value {
                EntityType::Mon => 0x01,
                EntityType::Mds => 0x02,
                EntityType::Osd => 0x04,
                EntityType::Client => 0x08,
                EntityType::Mgr => 0x10,
                EntityType::Auth => 0x20,
                EntityType::Any => 0xFF,
            }
        }
    }
    impl TryFrom<u8> for EntityType {
        type Error = ();
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            let res = match value {
                0x01 => Self::Mon,
                0x02 => Self::Mds,
                0x04 => Self::Osd,
                0x08 => Self::Client,
                0x10 => Self::Mgr,
                0x20 => Self::Auth,
                0xFF => Self::Any,
                _ => return Err(()),
            };
            Ok(res)
        }
    }
}
pub mod frame {
    mod epilogue {
        pub struct Epilogue {
            pub late_flags: u8,
            pub crcs: [u32; 4],
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Epilogue {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Epilogue",
                    "late_flags",
                    &self.late_flags,
                    "crcs",
                    &&self.crcs,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Epilogue {
            #[inline]
            fn clone(&self) -> Epilogue {
                Epilogue {
                    late_flags: ::core::clone::Clone::clone(&self.late_flags),
                    crcs: ::core::clone::Clone::clone(&self.crcs),
                }
            }
        }
        impl Epilogue {
            pub const SERIALIZED_SIZE: usize = 17;
            pub fn write(&self, mut output: impl std::io::Write) -> std::io::Result<usize> {
                output.write_all(&[self.late_flags])?;
                for crc in self.crcs.iter().copied() {
                    output.write_all(&crc.to_le_bytes())?;
                }
                Ok(1 + 4 * 4)
            }
            pub fn parse(data: &[u8]) -> Result<Self, String> {
                if data.len() != 17 {
                    return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!(
                            "Expected 17 bytes of epilogue data, got {0}",
                            data.len()
                        ))
                    }));
                }
                let late_flags = data[0];
                let mut crcs = [0u32; 4];
                for (idx, chunk) in data[1..].chunks_exact(4).enumerate() {
                    let value = u32::from_le_bytes(chunk.try_into().unwrap());
                    crcs[idx] = value;
                }
                Ok(Self { late_flags, crcs })
            }
        }
    }
    mod frame {
        use std::num::NonZeroU8;
        use crate::frame::{
            epilogue::Epilogue,
            preamble::{Preamble, SegmentDetail, Tag},
        };
        const ALGO: crc::Algorithm<u32> = crc::Algorithm {
            width: 32,
            poly: 0x1EDC6F41,
            init: u32::MAX,
            refin: true,
            refout: true,
            xorout: 0,
            check: 0,
            residue: 0,
        };
        const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&ALGO);
        const EMPTY: &'static [u8] = &[];
        pub struct Frame<'a> {
            tag: Tag,
            valid_segments: NonZeroU8,
            segments: [&'a [u8]; 4],
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for Frame<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Frame",
                    "tag",
                    &self.tag,
                    "valid_segments",
                    &self.valid_segments,
                    "segments",
                    &&self.segments,
                )
            }
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for Frame<'a> {
            #[inline]
            fn clone(&self) -> Frame<'a> {
                Frame {
                    tag: ::core::clone::Clone::clone(&self.tag),
                    valid_segments: ::core::clone::Clone::clone(&self.valid_segments),
                    segments: ::core::clone::Clone::clone(&self.segments),
                }
            }
        }
        impl<'a> Frame<'a> {
            pub fn new(tag: Tag, segments: &[&'a [u8]]) -> Option<Self> {
                if segments.len() == 0 || segments.len() > 4 {
                    return None;
                }
                let valid_segments = NonZeroU8::new(segments.len() as _).unwrap();
                let mut segments_out = [EMPTY; 4];
                segments_out[..segments.len()].copy_from_slice(segments);
                Some(Self {
                    tag,
                    valid_segments,
                    segments: segments_out,
                })
            }
            pub fn to_vec(&self) -> Vec<u8> {
                let mut output = Vec::new();
                self.write(&mut output).unwrap();
                output
            }
            pub fn write(&self, mut output: impl std::io::Write) -> std::io::Result<usize> {
                let segments = &self.segments[..self.valid_segments.get() as usize];
                let mut segment_details = [SegmentDetail::default(); 4];
                for (idx, segment) in segments.iter().enumerate() {
                    segment_details[idx] = SegmentDetail {
                        length: segment.len() as _,
                        alignment: 1,
                    };
                }
                let preamble = Preamble {
                    flags: 0,
                    tag: self.tag,
                    segment_count: self.valid_segments,
                    segment_details,
                    _reserved: 0,
                };
                let mut used = preamble.write(&mut output)?;
                let mut crcs = [0u32; 4];
                for (idx, segment) in segments.iter().enumerate() {
                    crcs[idx] = CRC.checksum(segment);
                    output.write_all(segment)?;
                    used += segment.len();
                }
                let epilogue = Epilogue {
                    late_flags: 0,
                    crcs,
                };
                used += epilogue.write(&mut output)?;
                Ok(used)
            }
            pub fn parse(preamble: &Preamble, data: &'a [u8]) -> Result<Self, String> {
                let mut trailer = data;
                let mut segments = [EMPTY; 4];
                for (idx, segment) in preamble.segments().iter().enumerate() {
                    let len = segment.len();
                    let (segment, left) = trailer.split_at_checked(len).ok_or_else(|| {
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!(
                                "Expected {0} bytes of segment data, but only had {1} left",
                                len,
                                trailer.len()
                            ))
                        })
                    })?;
                    trailer = left;
                    segments[idx] = segment;
                }
                let epilogue = Epilogue::parse(trailer)?;
                for (idx, crc) in epilogue.crcs.iter().copied().enumerate() {
                    if idx < preamble.segment_count.get() as usize {
                        let segment = segments[idx];
                        let calculated_crc = CRC.checksum(segment);
                        if crc != calculated_crc {
                            return Err(::alloc::__export::must_use({
                                :: alloc :: fmt :: format (format_args ! ("Found incorrect CRC 0x{0:08X} (expected 0x{1:08X}) for segment (#{2})" , crc , calculated_crc , idx + 1))
                            }));
                        }
                    } else {
                        if crc != 0 {
                            return Err(::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!(
                                    "Found non-zero CRC (0x{0:08X}) for a trailing segment (#{1}).",
                                    crc,
                                    idx + 1
                                ))
                            }));
                        }
                    }
                }
                Ok(Self {
                    tag: preamble.tag,
                    valid_segments: preamble.segment_count,
                    segments,
                })
            }
            pub const fn tag(&self) -> Tag {
                self.tag
            }
            pub fn segments(&self) -> &[&[u8]] {
                &self.segments[..self.valid_segments.get() as usize]
            }
        }
    }
    mod preamble {
        use std::num::NonZeroU8;
        use crate::frame::epilogue::Epilogue;
        /// The algorithm parameters used for the CRC
        /// calculated by Ceph.
        ///
        /// Note: these parameters do _not_ match the `crc32-c` (CASTAGNOLI)
        /// algorithm.
        const ALGO: crc::Algorithm<u32> = crc::Algorithm {
            width: 32,
            poly: 0x1EDC6F41,
            init: 0,
            refin: true,
            refout: true,
            xorout: 0,
            check: 0,
            residue: 0,
        };
        const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&ALGO);
        #[repr(u8)]
        pub enum Tag {
            Hello = 1,
            AuthRequest = 2,
            AuthBadMethod = 3,
            AuthReplyMore = 4,
            AuthRequestMore = 5,
            AuthDone = 6,
            AuthSignature = 7,
            ClientIdent = 8,
            ServerIdent = 9,
            IdentMissingFeatures = 10,
            SessionReconnect = 11,
            SessionReset = 12,
            SessionRetry = 13,
            SessionRetryGlobal = 14,
            SessionReconnectOk = 15,
            Wait = 16,
            Message = 17,
            Keepalive2 = 18,
            Keepalive2Ack = 19,
            Ack = 20,
            CompressionRequest = 21,
            CompressionDone = 22,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Tag {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Tag::Hello => "Hello",
                        Tag::AuthRequest => "AuthRequest",
                        Tag::AuthBadMethod => "AuthBadMethod",
                        Tag::AuthReplyMore => "AuthReplyMore",
                        Tag::AuthRequestMore => "AuthRequestMore",
                        Tag::AuthDone => "AuthDone",
                        Tag::AuthSignature => "AuthSignature",
                        Tag::ClientIdent => "ClientIdent",
                        Tag::ServerIdent => "ServerIdent",
                        Tag::IdentMissingFeatures => "IdentMissingFeatures",
                        Tag::SessionReconnect => "SessionReconnect",
                        Tag::SessionReset => "SessionReset",
                        Tag::SessionRetry => "SessionRetry",
                        Tag::SessionRetryGlobal => "SessionRetryGlobal",
                        Tag::SessionReconnectOk => "SessionReconnectOk",
                        Tag::Wait => "Wait",
                        Tag::Message => "Message",
                        Tag::Keepalive2 => "Keepalive2",
                        Tag::Keepalive2Ack => "Keepalive2Ack",
                        Tag::Ack => "Ack",
                        Tag::CompressionRequest => "CompressionRequest",
                        Tag::CompressionDone => "CompressionDone",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Tag {
            #[inline]
            fn clone(&self) -> Tag {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Tag {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Tag {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Tag {
            #[inline]
            fn eq(&self, other: &Tag) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        impl TryFrom<u8> for Tag {
            type Error = ();
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let value = match value {
                    1 => Self::Hello,
                    2 => Self::AuthRequest,
                    3 => Self::AuthBadMethod,
                    4 => Self::AuthReplyMore,
                    5 => Self::AuthRequestMore,
                    6 => Self::AuthDone,
                    7 => Self::AuthSignature,
                    8 => Self::ClientIdent,
                    9 => Self::ServerIdent,
                    10 => Self::IdentMissingFeatures,
                    11 => Self::SessionReconnect,
                    12 => Self::SessionReset,
                    13 => Self::SessionRetry,
                    14 => Self::SessionRetryGlobal,
                    15 => Self::SessionReconnectOk,
                    16 => Self::Wait,
                    17 => Self::Message,
                    18 => Self::Keepalive2,
                    19 => Self::Keepalive2Ack,
                    20 => Self::Ack,
                    21 => Self::CompressionRequest,
                    22 => Self::CompressionDone,
                    _ => return Err(()),
                };
                Ok(value)
            }
        }
        pub struct Preamble {
            pub(crate) tag: Tag,
            pub(crate) segment_count: NonZeroU8,
            pub(crate) segment_details: [SegmentDetail; 4],
            pub(crate) flags: u8,
            pub(crate) _reserved: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Preamble {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Preamble",
                    "tag",
                    &self.tag,
                    "segment_count",
                    &self.segment_count,
                    "segment_details",
                    &self.segment_details,
                    "flags",
                    &self.flags,
                    "_reserved",
                    &&self._reserved,
                )
            }
        }
        impl Preamble {
            pub const SERIALIZED_SIZE: usize = 32;
            pub fn data_and_epilogue_len(&self) -> usize {
                let segment_data: usize = self.segments().iter().map(|v| v.len()).sum();
                let epilogue_len = Epilogue::SERIALIZED_SIZE;
                segment_data + epilogue_len
            }
            pub fn write(&self, mut output: impl std::io::Write) -> std::io::Result<usize> {
                let mut buffer = [0u8; Self::SERIALIZED_SIZE];
                buffer[0] = self.tag as _;
                buffer[1] = self.segment_count.get();
                let mut used = 2;
                for (idx, detail) in self.segment_details.iter().enumerate() {
                    let start = used;
                    let end = start + 6;
                    used += 6;
                    let buffer = &mut buffer[start..end];
                    if idx < self.segment_count.get() as usize {
                        detail.write(buffer)?;
                    } else {
                        buffer.copy_from_slice(&[0u8; 6]);
                    }
                }
                buffer[used] = self.flags;
                used += 1;
                buffer[used] = self._reserved;
                used += 1;
                match (&used, &28) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                let crc = CRC.checksum(&buffer[..used]);
                buffer[used..used + 4].copy_from_slice(&crc.to_le_bytes());
                output.write_all(&buffer)?;
                used += 4;
                Ok(used)
            }
            pub fn parse(input: &[u8]) -> Result<Self, String> {
                if input.len() != Self::SERIALIZED_SIZE {
                    return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!(
                            "Expected 32 bytes of preamble data, got {0}",
                            input.len()
                        ))
                    }));
                }
                let (tag_scount, buffer) = input.split_at(2);
                let Ok(tag) = Tag::try_from(tag_scount[0]) else {
                    return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("Unknown tag value {0}", tag_scount[0]))
                    }));
                };
                let Some(segment_count) = NonZeroU8::new(tag_scount[1]) else {
                    return Err("Segment count was zero".to_string());
                };
                if segment_count.get() > 4 {
                    return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!(
                            "Segment count was greater than 4 ({0})",
                            segment_count
                        ))
                    }));
                }
                let (chunks, rest) = buffer.split_at(6 * 4);
                let (chunks, _) = chunks.as_chunks::<6>();
                let mut segment_details = [SegmentDetail::default(); 4];
                for i in 0..(segment_count.get() as usize) {
                    segment_details[i] = SegmentDetail::parse(chunks[i]);
                }
                let flags = rest[0];
                let _reserved = rest[1];
                let crc = <[u8; 4]>::try_from(&rest[2..]).unwrap();
                let crc = u32::from_le_bytes(crc);
                let calculated_crc = CRC.checksum(&input[..28]);
                if calculated_crc != crc {
                    return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!(
                            "Preamble CRC mismatch (received: 0x{0:08X}, calculated: 0x{1:08X}",
                            crc, calculated_crc
                        ))
                    }));
                }
                Ok(Self {
                    tag,
                    segment_count,
                    segment_details,
                    flags,
                    _reserved,
                })
            }
            pub(crate) fn segments(&self) -> &[SegmentDetail] {
                &self.segment_details[..self.segment_count.get() as usize]
            }
        }
        pub(crate) struct SegmentDetail {
            pub length: u32,
            pub alignment: u16,
        }
        #[automatically_derived]
        impl ::core::default::Default for SegmentDetail {
            #[inline]
            fn default() -> SegmentDetail {
                SegmentDetail {
                    length: ::core::default::Default::default(),
                    alignment: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SegmentDetail {
            #[inline]
            fn clone(&self) -> SegmentDetail {
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<u16>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for SegmentDetail {}
        #[automatically_derived]
        impl ::core::fmt::Debug for SegmentDetail {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SegmentDetail",
                    "length",
                    &self.length,
                    "alignment",
                    &&self.alignment,
                )
            }
        }
        impl SegmentDetail {
            pub fn parse(buffer: [u8; 6]) -> Self {
                let length = <[u8; 4]>::try_from(&buffer[..4]).unwrap();
                let alignment = <[u8; 2]>::try_from(&buffer[4..]).unwrap();
                Self {
                    length: u32::from_le_bytes(length),
                    alignment: u16::from_le_bytes(alignment),
                }
            }
            pub fn write(&self, mut output: impl std::io::Write) -> std::io::Result<usize> {
                output.write_all(&self.length.to_le_bytes())?;
                output.write_all(&self.alignment.to_le_bytes())?;
                Ok(6)
            }
            pub fn len(&self) -> usize {
                self.length as _
            }
            pub fn alignment(&self) -> usize {
                self.alignment as _
            }
        }
    }
    pub use frame::Frame;
    pub use preamble::{Preamble, Tag};
}
pub mod messages {
    pub mod auth {
        mod done {
            use crate::messages::auth::ConMode;
            pub struct AuthDone {
                pub global_id: u64,
                pub connection_mode: ConMode,
                pub auth_payload: Vec<u8>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for AuthDone {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "AuthDone",
                        "global_id",
                        &self.global_id,
                        "connection_mode",
                        &self.connection_mode,
                        "auth_payload",
                        &&self.auth_payload,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for AuthDone {
                #[inline]
                fn clone(&self) -> AuthDone {
                    AuthDone {
                        global_id: ::core::clone::Clone::clone(&self.global_id),
                        connection_mode: ::core::clone::Clone::clone(&self.connection_mode),
                        auth_payload: ::core::clone::Clone::clone(&self.auth_payload),
                    }
                }
            }
            impl AuthDone {
                pub fn parse(data: &[u8]) -> Result<Self, String> {
                    if data.len() < 16 {
                        return Err(::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!(
                                "Expected at least 16 bytes of auth done data, got only {0}",
                                data.len()
                            ))
                        }));
                    }
                    let global_id = u64::from_le_bytes(data[0..8].try_into().unwrap());
                    let connection_mode = u32::from_le_bytes(data[8..12].try_into().unwrap());
                    let Ok(Ok(connection_mode)) =
                        u8::try_from(connection_mode).map(ConMode::try_from)
                    else {
                        return Err(::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!(
                                "Unknown connection mode {0}",
                                connection_mode
                            ))
                        }));
                    };
                    let payload_bytes = u32::from_le_bytes(data[12..16].try_into().unwrap());
                    if data[16..].len() as u32 != payload_bytes {
                        return Err(::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!(
                                "Expected {0} bytes of auth payload data, got only {1}",
                                payload_bytes,
                                data[16..].len()
                            ))
                        }));
                    }
                    let auth_payload = data[16..].to_vec();
                    Ok(Self {
                        global_id,
                        connection_mode,
                        auth_payload,
                    })
                }
            }
        }
        mod request {
            use crate::{
                Encode, EntityName,
                messages::auth::{AuthMethod, ConMode},
            };
            pub trait AuthRequestPayload: crate::sealed::Sealed {
                const METHOD: AuthMethod;
                fn payload(&self) -> Vec<u8>;
            }
            pub struct AuthRequest {
                method: AuthMethod,
                preferred_modes: Vec<u32>,
                auth_payload: Vec<u8>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for AuthRequest {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "AuthRequest",
                        "method",
                        &self.method,
                        "preferred_modes",
                        &self.preferred_modes,
                        "auth_payload",
                        &&self.auth_payload,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for AuthRequest {
                #[inline]
                fn clone(&self) -> AuthRequest {
                    AuthRequest {
                        method: ::core::clone::Clone::clone(&self.method),
                        preferred_modes: ::core::clone::Clone::clone(&self.preferred_modes),
                        auth_payload: ::core::clone::Clone::clone(&self.auth_payload),
                    }
                }
            }
            impl AuthRequest {
                pub fn new<T>(auth_method: T, preferred_modes: Vec<ConMode>) -> Self
                where
                    T: AuthRequestPayload,
                {
                    let preferred_modes = preferred_modes
                        .into_iter()
                        .map(|v| u8::from(v) as u32)
                        .collect();
                    Self {
                        method: T::METHOD,
                        preferred_modes,
                        auth_payload: auth_method.payload(),
                    }
                }
            }
            impl Encode for AuthRequest {
                fn encode(&self, buffer: &mut Vec<u8>) {
                    (u8::from(self.method) as u32).encode(buffer);
                    self.preferred_modes.encode(buffer);
                    self.auth_payload.encode(buffer);
                }
            }
            pub struct AuthMethodNone {
                pub name: EntityName,
                pub global_id: u64,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for AuthMethodNone {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "AuthMethodNone",
                        "name",
                        &self.name,
                        "global_id",
                        &&self.global_id,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for AuthMethodNone {
                #[inline]
                fn clone(&self) -> AuthMethodNone {
                    AuthMethodNone {
                        name: ::core::clone::Clone::clone(&self.name),
                        global_id: ::core::clone::Clone::clone(&self.global_id),
                    }
                }
            }
            impl crate::sealed::Sealed for AuthMethodNone {}
            impl AuthRequestPayload for AuthMethodNone {
                const METHOD: AuthMethod = AuthMethod::None;
                fn payload(&self) -> Vec<u8> {
                    let mut buffer = Vec::with_capacity(9);
                    buffer.push(1u8);
                    self.name.encode(&mut buffer);
                    self.global_id.encode(&mut buffer);
                    buffer
                }
            }
        }
        mod signature {
            use crate::Encode;
            pub struct AuthSignature {
                pub sha256: [u8; 32],
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for AuthSignature {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "AuthSignature",
                        "sha256",
                        &&self.sha256,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for AuthSignature {
                #[inline]
                fn clone(&self) -> AuthSignature {
                    AuthSignature {
                        sha256: ::core::clone::Clone::clone(&self.sha256),
                    }
                }
            }
            impl Encode for AuthSignature {
                fn encode(&self, buffer: &mut Vec<u8>) {
                    self.sha256.encode(buffer);
                }
            }
            impl AuthSignature {
                pub fn parse(data: &[u8]) -> Result<Self, String> {
                    if data.len() != 32 {
                        return Err(::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!(
                                "Expected {0} bytes of signature data, got only {1}",
                                32,
                                data.len()
                            ))
                        }));
                    }
                    let sha256 = data.try_into().unwrap();
                    Ok(Self { sha256 })
                }
            }
        }
        pub use done::AuthDone;
        pub use request::{AuthMethodNone, AuthRequest, AuthRequestPayload};
        pub use signature::AuthSignature;
        pub enum AuthMethod {
            Unknown = 0,
            None = 1,
            CephX = 2,
            Gss = 4,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AuthMethod {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        AuthMethod::Unknown => "Unknown",
                        AuthMethod::None => "None",
                        AuthMethod::CephX => "CephX",
                        AuthMethod::Gss => "Gss",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for AuthMethod {
            #[inline]
            fn clone(&self) -> AuthMethod {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for AuthMethod {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for AuthMethod {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for AuthMethod {
            #[inline]
            fn eq(&self, other: &AuthMethod) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for AuthMethod {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl From<AuthMethod> for u8 {
            fn from(value: AuthMethod) -> Self {
                value as _
            }
        }
        impl TryFrom<u8> for AuthMethod {
            type Error = ();
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let res = match value {
                    0 => Self::Unknown,
                    1 => Self::None,
                    2 => Self::CephX,
                    3 => Self::Gss,
                    _ => return Err(()),
                };
                Ok(res)
            }
        }
        pub enum ConMode {
            Unknown = 0,
            Crc = 1,
            Secure = 2,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ConMode {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ConMode::Unknown => "Unknown",
                        ConMode::Crc => "Crc",
                        ConMode::Secure => "Secure",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ConMode {
            #[inline]
            fn clone(&self) -> ConMode {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ConMode {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ConMode {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ConMode {
            #[inline]
            fn eq(&self, other: &ConMode) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ConMode {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl From<ConMode> for u8 {
            fn from(value: ConMode) -> Self {
                value as _
            }
        }
        impl TryFrom<u8> for ConMode {
            type Error = ();
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let res = match value {
                    0 => Self::Unknown,
                    1 => Self::Crc,
                    2 => Self::Secure,
                    _ => return Err(()),
                };
                Ok(res)
            }
        }
    }
    mod banner {
        use crate::messages::MsgrFeatures;
        pub struct Banner {
            supported_features: MsgrFeatures,
            required_features: MsgrFeatures,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Banner {
            #[inline]
            fn clone(&self) -> Banner {
                let _: ::core::clone::AssertParamIsClone<MsgrFeatures>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Banner {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Banner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Banner",
                    "supported_features",
                    &self.supported_features,
                    "required_features",
                    &&self.required_features,
                )
            }
        }
        impl Default for Banner {
            fn default() -> Self {
                let features = MsgrFeatures(0x0);
                Self {
                    supported_features: features,
                    required_features: features,
                }
            }
        }
        const HEADER: &'static [u8] = b"ceph v2\n";
        impl Banner {
            pub const SERIALIZED_SIZE: usize = 26;
            pub fn new(supported_features: MsgrFeatures, required_features: MsgrFeatures) -> Self {
                Self {
                    supported_features,
                    required_features,
                }
            }
            pub fn parse(data: &[u8; Self::SERIALIZED_SIZE]) -> Result<Self, String> {
                let (header, data) = data.split_at(10);
                if &header[..8] != HEADER {
                    return Err("Header is not correct".into());
                }
                let data_len = u16::from_le_bytes([header[8], header[9]]) as usize;
                if data.len() != data_len {
                    return Err("data length mismatch".into());
                }
                let (supported_features, data) = data
                    .split_first_chunk::<8>()
                    .expect("8 bytes of supported feature data");
                let supported_features = MsgrFeatures(u64::from_le_bytes(*supported_features));
                let (required_features, _) = data
                    .split_first_chunk::<8>()
                    .expect("8 bytes of required feature data");
                let required_features = MsgrFeatures(u64::from_le_bytes(*required_features));
                Ok(Self {
                    required_features,
                    supported_features,
                })
            }
            pub fn write<'a>(&self, output: &'a mut [u8; Self::SERIALIZED_SIZE]) {
                output[..8].copy_from_slice(HEADER);
                output[8..10].copy_from_slice(&16u16.to_le_bytes());
                output[10..18].copy_from_slice(&self.supported_features.0.to_le_bytes());
                output[18..26].copy_from_slice(&self.required_features.0.to_le_bytes());
            }
            pub fn supported(&self) -> &MsgrFeatures {
                &self.supported_features
            }
            pub fn required(&self) -> &MsgrFeatures {
                &self.required_features
            }
        }
    }
    mod client_ident {
        use crate::{CephFeatureSet, Encode, entity_address::EntityAddress};
        pub struct ClientIdent {
            pub addresses: Vec<EntityAddress>,
            pub target: EntityAddress,
            pub gid: i64,
            pub global_seq: u64,
            /// The supported `ceph` features. (_Not_ `msgr2` features).
            pub supported_features: CephFeatureSet,
            /// The required `ceph` features. (_Not_ `msgr2` features).
            pub required_features: CephFeatureSet,
            pub flags: u64,
            pub cookie: u64,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ClientIdent {
            #[inline]
            fn clone(&self) -> ClientIdent {
                ClientIdent {
                    addresses: ::core::clone::Clone::clone(&self.addresses),
                    target: ::core::clone::Clone::clone(&self.target),
                    gid: ::core::clone::Clone::clone(&self.gid),
                    global_seq: ::core::clone::Clone::clone(&self.global_seq),
                    supported_features: ::core::clone::Clone::clone(&self.supported_features),
                    required_features: ::core::clone::Clone::clone(&self.required_features),
                    flags: ::core::clone::Clone::clone(&self.flags),
                    cookie: ::core::clone::Clone::clone(&self.cookie),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ClientIdent {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "addresses",
                    "target",
                    "gid",
                    "global_seq",
                    "supported_features",
                    "required_features",
                    "flags",
                    "cookie",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.addresses,
                    &self.target,
                    &self.gid,
                    &self.global_seq,
                    &self.supported_features,
                    &self.required_features,
                    &self.flags,
                    &&self.cookie,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "ClientIdent", names, values)
            }
        }
        impl Encode for ClientIdent {
            fn encode(&self, buffer: &mut Vec<u8>) {
                buffer.push(2u8);
                self.addresses.encode(buffer);
                self.target.encode(buffer);
                self.gid.encode(buffer);
                self.global_seq.encode(buffer);
                self.supported_features.encode(buffer);
                self.required_features.encode(buffer);
                self.flags.encode(buffer);
                self.cookie.encode(buffer);
            }
        }
        impl ClientIdent {
            pub(crate) fn parse(data: &[u8]) -> Result<Self, String> {
                if data.len() < 5 {
                    return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!(
                            "Need at least 5 bytes for client ident, only got {0}",
                            data.len()
                        ))
                    }));
                }
                if data[0] != 2 {
                    return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!(
                            "Unsupported addrvec version {0}",
                            data[0]
                        ))
                    }));
                }
                let len = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
                let mut left = &data[5..];
                let mut addresses = Vec::with_capacity(len as _);
                for _ in 0..len {
                    let (used, address) = EntityAddress::parse(left)?;
                    left = &left[used..];
                    addresses.push(address);
                }
                let (used, target) = EntityAddress::parse(left)?;
                left = &left[used..];
                if left.len() < 48 {
                    return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!(
                            "Need at least 48 leftover data bytes for client ident, got only {0}",
                            left.len()
                        ))
                    }));
                }
                let gid = i64::from_le_bytes(left[0..8].try_into().unwrap());
                let global_seq = u64::from_le_bytes(left[8..16].try_into().unwrap());
                let supported_features = u64::from_le_bytes(left[16..24].try_into().unwrap());
                let required_features = u64::from_le_bytes(left[24..32].try_into().unwrap());
                let flags = u64::from_le_bytes(left[32..40].try_into().unwrap());
                let cookie = u64::from_le_bytes(left[40..48].try_into().unwrap());
                let supported_features = CephFeatureSet {
                    bits: supported_features,
                    mask: supported_features,
                };
                let required_features = CephFeatureSet {
                    bits: required_features,
                    mask: required_features,
                };
                Ok(Self {
                    addresses,
                    target,
                    gid,
                    global_seq,
                    supported_features,
                    required_features,
                    flags,
                    cookie,
                })
            }
        }
    }
    mod hello {
        use crate::{Encode, EntityType, entity_address::EntityAddress};
        pub struct Hello {
            /// The type of entity we are communicating with.
            pub entity_type: EntityType,
            /// The peer address that the entity we are communicating
            /// with observes us to have.
            pub peer_address: EntityAddress,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Hello {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Hello",
                    "entity_type",
                    &self.entity_type,
                    "peer_address",
                    &&self.peer_address,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Hello {
            #[inline]
            fn clone(&self) -> Hello {
                Hello {
                    entity_type: ::core::clone::Clone::clone(&self.entity_type),
                    peer_address: ::core::clone::Clone::clone(&self.peer_address),
                }
            }
        }
        impl Encode for Hello {
            fn encode(&self, buffer: &mut Vec<u8>) {
                buffer.push(self.entity_type.into());
                self.peer_address.encode(buffer);
            }
        }
        impl Hello {
            pub fn parse(data: &[u8]) -> Result<Self, String> {
                let entity_type = EntityType::try_from(data[0]).map_err(|_| {
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("Unknown entity type {0}", data[0]))
                    })
                })?;
                let (_, address) = EntityAddress::parse(&data[1..])?;
                Ok(Self {
                    entity_type,
                    peer_address: address,
                })
            }
        }
    }
    mod ident_missing_features {
        use crate::{Encode, messages::MsgrFeatures};
        pub struct IdentMissingFeatures {
            pub features: MsgrFeatures,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdentMissingFeatures {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "IdentMissingFeatures",
                    "features",
                    &&self.features,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdentMissingFeatures {
            #[inline]
            fn clone(&self) -> IdentMissingFeatures {
                IdentMissingFeatures {
                    features: ::core::clone::Clone::clone(&self.features),
                }
            }
        }
        impl Encode for IdentMissingFeatures {
            fn encode(&self, buffer: &mut Vec<u8>) {
                self.features.encode(buffer);
            }
        }
        impl IdentMissingFeatures {
            pub fn parse(data: &[u8]) -> Option<Self> {
                if data.len() != 8 {
                    return None;
                }
                let features = MsgrFeatures(u64::from_le_bytes(data.try_into().unwrap()));
                Some(Self { features })
            }
        }
    }
    mod keepalive {
        use crate::{Encode, messages::Timestamp};
        pub struct Keepalive {
            pub timestamp: Timestamp,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Keepalive {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Keepalive",
                    "timestamp",
                    &&self.timestamp,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Keepalive {
            #[inline]
            fn clone(&self) -> Keepalive {
                Keepalive {
                    timestamp: ::core::clone::Clone::clone(&self.timestamp),
                }
            }
        }
        impl Encode for Keepalive {
            fn encode(&self, buffer: &mut Vec<u8>) {
                self.timestamp.encode(buffer);
            }
        }
        pub struct KeepaliveAck {}
        #[automatically_derived]
        impl ::core::fmt::Debug for KeepaliveAck {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "KeepaliveAck")
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for KeepaliveAck {
            #[inline]
            fn clone(&self) -> KeepaliveAck {
                KeepaliveAck {}
            }
        }
    }
    pub use banner::Banner;
    pub use client_ident::ClientIdent;
    pub use hello::Hello;
    pub use ident_missing_features::IdentMissingFeatures;
    pub use keepalive::{Keepalive, KeepaliveAck};
    use crate::Encode;
    const FEATURE_REVISION_21: u64 = 1 << 0;
    const FEATURE_COMPRESSION: u64 = 1 << 1;
    pub struct MsgrFeatures(u64);
    #[automatically_derived]
    impl ::core::fmt::Debug for MsgrFeatures {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "MsgrFeatures", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MsgrFeatures {
        #[inline]
        fn clone(&self) -> MsgrFeatures {
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MsgrFeatures {}
    impl MsgrFeatures {
        pub const fn empty() -> Self {
            Self(0)
        }
        pub const fn revision_21(&self) -> bool {
            self.0 & FEATURE_REVISION_21 == FEATURE_REVISION_21
        }
        pub const fn set_revision_21(&mut self, revision_21: bool) {
            if !revision_21 {
                self.0 &= !FEATURE_REVISION_21;
            } else {
                self.0 |= FEATURE_REVISION_21;
            }
        }
        pub const fn compression(&self) -> bool {
            self.0 & FEATURE_COMPRESSION == FEATURE_COMPRESSION
        }
        pub fn set_compression(&mut self, compression: bool) {
            if !compression {
                self.0 &= !FEATURE_COMPRESSION
            } else {
                self.0 |= FEATURE_COMPRESSION
            }
        }
    }
    impl Encode for MsgrFeatures {
        fn encode(&self, buffer: &mut Vec<u8>) {
            self.0.encode(buffer);
        }
    }
    pub struct Timestamp {
        pub tv_sec: u32,
        pub tv_nsec: u32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Timestamp {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Timestamp",
                "tv_sec",
                &self.tv_sec,
                "tv_nsec",
                &&self.tv_nsec,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Timestamp {
        #[inline]
        fn clone(&self) -> Timestamp {
            Timestamp {
                tv_sec: ::core::clone::Clone::clone(&self.tv_sec),
                tv_nsec: ::core::clone::Clone::clone(&self.tv_nsec),
            }
        }
    }
    impl Encode for Timestamp {
        fn encode(&self, buffer: &mut Vec<u8>) {
            self.tv_sec.encode(buffer);
            self.tv_nsec.encode(buffer);
        }
    }
    impl Timestamp {
        pub fn parse(buffer: &mut [u8]) -> Option<(Self, usize)> {
            if buffer.len() < 8 {
                return None;
            }
            let tv_sec = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
            let tv_nsec = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
            Some((Self { tv_sec, tv_nsec }, 8))
        }
    }
}
pub use ceph_features::CephFeatureSet;
pub use connection::{Connection, Message};
pub use encode::Encode;
pub use entity_address::{EntityAddress, EntityAddressType};
pub use entity_name::EntityName;
pub use entity_type::EntityType;
mod sealed {
    pub trait Sealed {}
}
