use crate::{DecodeError, Encode};

/// A ceph feature set, describing which features an entity
/// supports.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CephFeatureSet {
    pub(crate) bits: u64,
    pub(crate) mask: u64,
}

impl Encode for CephFeatureSet {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.bits.encode(buffer);
    }
}

macro_rules ! define_features {
    ($(($qual:tt: $bit:literal, $incarnation: ident, $name:ident);)*) => {
        impl CephFeatureSet {
            $(
                feature!($qual: $bit, $incarnation, $name);
            )*

            /// A list of all available [`CephFeatureSet`]s
            pub const LIST: &'static [CephFeatureSet] = &[
                $(
                    CephFeatureSet::$name,
                )*
            ];

            /// A [`CephFeatureSet`] that contains all features.
            pub const ALL: CephFeatureSet = CephFeatureSet::EMPTY $(.union(CephFeatureSet::$name))*;
        }

        impl core::fmt::Display for CephFeatureSet {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut output_any = false;

                $(
                    if CephFeatureSet::$name != CephFeatureSet::EMPTY {
                        if self.contains(&CephFeatureSet::$name) {
                            if !output_any {
                                write!(f, "{}", stringify!($name))?;
                            } else {
                                write!(f, " | {}", stringify!($name))?;
                            }
                            output_any = true;
                        }
                    }
                )*

                let _ = output_any;

                Ok(())
            }
        }
    }
}

macro_rules! feature {
    (a: $bit:literal, $incarnation:ident, $name:ident) => {
        #[expect(missing_docs)]
        pub const $name: CephFeatureSet = CephFeatureSet {
            bits: 1 << $bit,
            mask: 1 << $bit | Self::$incarnation,
        };
    };

    (d: $bit:literal, $incarnation:ident, $name:ident) => {
        feature!($bit, $incarnation, $name, $mask)
    };

    (r: $bit:literal, $incarnation:ident, $name:ident) => {
        #[allow(unused)]
        const $name: CephFeatureSet = CephFeatureSet::EMPTY;
    };
}

define_features! {
    (a:  0, INCARNATION_1, UID);
    (a:  1, INCARNATION_1, NOSRCADDR);        // 2.6.35 req
    (r:  2, INCARNATION_1, MONCLOCKCHECK);
    (a:  2, INCARNATION_3, SERVER_NAUTILUS);
    (a:  3, INCARNATION_1, FLOCK);            // 2.6.36
    (a:  4, INCARNATION_1, SUBSCRIBE2);       // 4.6 req
    (a:  5, INCARNATION_1, MONNAMES);
    (a:  6, INCARNATION_1, RECONNECT_SEQ);    // 3.10 req
    (a:  7, INCARNATION_1, DIRLAYOUTHASH);    // 2.6.38
    (a:  8, INCARNATION_1, OBJECTLOCATOR);
    (a:  9, INCARNATION_1, PGID64);           // 3.9 req
    (a: 10, INCARNATION_1, INCSUBOSDMAP);
    (a: 11, INCARNATION_1, PGPOOL3);          // 3.9 req
    (a: 12, INCARNATION_1, OSDREPLYMUX);
    (a: 13, INCARNATION_1, OSDENC);           // 3.9 req
    (r: 14, INCARNATION_1, OMAP);
    (a: 14, INCARNATION_2, SERVER_KRAKEN);
    (a: 15, INCARNATION_1, MONENC);
    (r: 16, INCARNATION_1, QUERY_T);
    (a: 16, INCARNATION_3, SERVER_OCTOPUS);
    (a: 16, INCARNATION_3, OSD_REPOP_MLCOD);
    (r: 17, INCARNATION_1, INDEP_PG_MAP);
    (a: 17, INCARNATION_3, OS_PERF_STAT_NS);
    (a: 18, INCARNATION_1, CRUSH_TUNABLES);   // 3.6
    (r: 19, INCARNATION_1, CHUNKY_SCRUB);
    (a: 19, INCARNATION_2, OSD_PGLOG_HARDLIMIT);
    (r: 20, INCARNATION_1, MON_NULLROUTE);
    (a: 20, INCARNATION_3, SERVER_PACIFIC);
    (r: 21, INCARNATION_1, MON_GV);
    (a: 21, INCARNATION_2, SERVER_LUMINOUS);  // 4.13
    (a: 21, INCARNATION_2, RESEND_ON_SPLIT);  // overlap
    (a: 21, INCARNATION_2, RADOS_BACKOFF);    // overlap
    (a: 21, INCARNATION_2, OSDMAP_PG_UPMAP);  // overlap
    (a: 21, INCARNATION_2, CRUSH_CHOOSE_ARGS); // overlap
    (r: 22, INCARNATION_1, BACKFILL_RESERVATION);
    (a: 22, INCARNATION_2, OSD_FIXED_COLLECTION_LIST);
    (a: 23, INCARNATION_1, MSG_AUTH);         // 3.19 req (unless nocephx_require_signatures);
    (r: 24, INCARNATION_1, RECOVERY_RESERVATION);
    (a: 24, INCARNATION_2, RECOVERY_RESERVATION_2);
    (a: 25, INCARNATION_1, CRUSH_TUNABLES2);  // 3.9
    (a: 26, INCARNATION_1, CREATEPOOLID);
    (a: 27, INCARNATION_1, REPLY_CREATE_INODE); // 3.9
    (r: 28, INCARNATION_1, OSD_HBMSGS);
    (a: 28, INCARNATION_2, SERVER_MIMIC);
    (a: 29, INCARNATION_1, MDSENC);           // 4.7
    (a: 30, INCARNATION_1, OSDHASHPSPOOL);    // 3.9
    (r: 31, INCARNATION_1, MON_SINGLE_PAXOS);
    (a: 31, INCARNATION_3, SERVER_REEF);
    (r: 32, INCARNATION_1, OSD_SNAPMAPPER);
    (a: 32, INCARNATION_3, STRETCH_MODE);
    (r: 33, INCARNATION_1, MON_SCRUB);
    (a: 33, INCARNATION_3, SERVER_QUINCY);
    (r: 34, INCARNATION_1, OSD_PACKED_RECOVERY);
    (a: 34, INCARNATION_3, RANGE_BLOCKLIST);
    (a: 35, INCARNATION_1, OSD_CACHEPOOL);    // 3.14
    (a: 36, INCARNATION_1, CRUSH_V2);         // 3.14
    (a: 37, INCARNATION_1, EXPORT_PEER);      // 3.14
    (a: 38, INCARNATION_2, CRUSH_MSR);        // X.XX kernel version once in a release
    (a: 39, INCARNATION_1, OSDMAP_ENC);       // 3.15
    (a: 40, INCARNATION_1, MDS_INLINE_DATA);  // 3.19
    (a: 41, INCARNATION_1, CRUSH_TUNABLES3);  // 3.15
    (a: 41, INCARNATION_1, OSD_PRIMARY_AFFINITY); // overlap
    (a: 42, INCARNATION_1, MSGR_KEEPALIVE2);  // 4.3 (for consistency);
    (a: 43, INCARNATION_1, OSD_POOLRESEND);   // 4.13
    (a: 44, INCARNATION_2, NVMEOFHA);
    (r: 45, INCARNATION_1, OSD_SET_ALLOC_HINT);
    (a: 45, INCARNATION_2, NVMEOFHAMAP);
    // available
    (a: 46, INCARNATION_1, OSD_FADVISE_FLAGS);
    (r: 46, INCARNATION_1, OSD_REPOP); // overlap
    (r: 46, INCARNATION_1, OSD_OBJECT_DIGEST); // overlap
    (r: 46, INCARNATION_1, OSD_TRANSACTION_MAY_LAYOUT); // overlap
    (a: 47, INCARNATION_1, MDS_QUOTA);        // 4.17
    (a: 48, INCARNATION_1, CRUSH_V4);         // 4.1
    (r: 49, INCARNATION_1, OSD_MIN_SIZE_RECOVERY);
    (r: 49, INCARNATION_1, OSD_PROXY_FEATURES); // overlap
    (a: 49, INCARNATION_2, SERVER_SQUID);
    (r: 50, INCARNATION_1, MON_METADATA);
    (a: 50, INCARNATION_2, SERVER_TENTACLE);
    (r: 51, INCARNATION_1, OSD_BITWISE_HOBJ_SORT);
    // available
    (r: 52, INCARNATION_1, OSD_PROXY_WRITE_FEATURES);
    // available
    (r: 53, INCARNATION_1, ERASURE_CODE_PLUGINS_V3);
    // available
    (r: 54, INCARNATION_1, OSD_HITSET_GMT);
    // available
    (r: 55, INCARNATION_1, HAMMER_0_94_4);
    // available
    (a: 56, INCARNATION_1, NEW_OSDOP_ENCODING); // 4.13 (for pg_pool_t >= v25);
    (a: 57, INCARNATION_1, MON_STATEFUL_SUB); // 4.13
    (r: 57, INCARNATION_1, MON_ROUTE_OSDMAP); // overlap
    (a: 57, INCARNATION_1, SERVER_JEWEL); // overlap
    (a: 58, INCARNATION_1, CRUSH_TUNABLES5);  // 4.5
    (a: 58, INCARNATION_1, NEW_OSDOPREPLY_ENCODING); // overlap
    (a: 58, INCARNATION_1, FS_FILE_LAYOUT_V2); // overlap
    (a: 59, INCARNATION_1, FS_BTIME);
    (a: 59, INCARNATION_1, FS_CHANGE_ATTR); // overlap
    (a: 59, INCARNATION_1, MSG_ADDR2); // overlap
    (a: 60, INCARNATION_1, OSD_RECOVERY_DELETES); // *do not share this bit*
    (a: 61, INCARNATION_1, CEPHX_V2);         // 4.19, *do not share this bit*

    (a: 62, INCARNATION_1, RESERVED);           // do not use; used as a sentinel
    (r: 63, INCARNATION_1, RESERVED_BROKEN); // client-facing
    // available
}

impl TryFrom<u64> for CephFeatureSet {
    type Error = DecodeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self {
            bits: value,
            mask: value,
        })
    }
}

impl From<&CephFeatureSet> for u64 {
    fn from(value: &CephFeatureSet) -> Self {
        value.bits
    }
}

impl CephFeatureSet {
    const INCARNATION_1: u64 = 0;
    const INCARNATION_2: u64 = 1 << 57;
    const INCARNATION_3: u64 = 1 << 57 | 1 << 28;

    /// The empty [`CephFeatureSet`].
    pub const EMPTY: CephFeatureSet = CephFeatureSet { bits: 0, mask: 0 };

    /// Combine this [`CephFeatureSet`] with another, yielding the union of the two.
    pub const fn union(self, rhs: Self) -> Self {
        Self {
            bits: self.bits | rhs.bits,
            mask: self.mask | rhs.mask,
        }
    }

    /// Combine this [`CephFeatureSet`] with another, yielding the intersection of the two.
    pub const fn intersection(self, rhs: Self) -> Self {
        Self {
            bits: self.bits & rhs.bits,
            mask: self.mask & rhs.mask,
        }
    }

    /// Check whether this [`CephFeatureSet`] contains and supports
    /// all features in `features`.
    pub fn contains(&self, features: &CephFeatureSet) -> bool {
        self.bits & features.mask == features.mask
    }
}

impl core::ops::BitOr for CephFeatureSet {
    type Output = CephFeatureSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
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
        self.intersection(rhs)
    }
}

impl core::ops::BitAndAssign for CephFeatureSet {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

#[test]
fn all_has_all() {
    for feature in CephFeatureSet::LIST {
        assert!(CephFeatureSet::ALL.contains(feature), "{feature:?}");
    }
}

#[test]
fn empty_has_none() {
    for feature in CephFeatureSet::LIST {
        assert!(!CephFeatureSet::EMPTY.contains(feature), "{feature:?}");
    }
}

#[test]
fn all_have_empty() {
    for feature in CephFeatureSet::LIST {
        assert!(feature.contains(&CephFeatureSet::EMPTY), "{feature:?}");
    }
}

#[test]
fn has_combination() {
    let f1 = CephFeatureSet::UID;
    let f2 = CephFeatureSet::MONNAMES;
    let f3 = CephFeatureSet::CREATEPOOLID;
    let combined = f1 | f2;

    assert!(combined.contains(&f1));
    assert!(combined.contains(&f2));
    assert!(!combined.contains(&f3));
}
