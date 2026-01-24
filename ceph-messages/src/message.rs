use crate::*;

macro_rules! msg_type {
    ($($n:ident$(($ty:ident))? = $v:literal,)*)  => {
        #[derive(Debug, Clone)]
        #[repr(u16)]
        pub enum CephMessage {
            $(
                $n$(($ty))? = $v,
            )*
        }

        impl CephMessage {
            pub fn decode_message(ty: u16, segments: &[&[u8]]) -> Result<Self, DecodeMessageError> {
                match ty {
                    $(
                        $v => Ok(Self::$n$(($ty::decode_message(segments)?))?),
                    )*
                    ty => Err(DecodeMessageError::Custom(format!("Unknown message type: {ty}")))
                }
            }

            pub fn identifier(&self) -> u16 {
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                match self {
                    $(
                        Self::$n$(($ty))? => $v,
                    )*
                }
            }
        }
    };
}

msg_type! {
    ShutDown = 1,
    Ping = 2,
    MonMap(MonMap) = 4,
    MonGetMap = 5,
    MonGetOsdMap = 6,
    MonMetadata = 7,
    StatFs = 13,
    StatFsReply = 14,
    MonSubscribe(MonSubscribe) = 15,
    MonSubscribeAck = 16,
    Auth = 17,
    AuthReply = 18,
    MonGetVersion = 19,
    MonGetVersionReply = 20,
    OsdMap = 41,
    GetPoolStats = 58,
    GetPoolStatsReply = 59,
    Config(Config) = 62,
}
