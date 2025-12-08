#[allow(warnings)]
mod librados;

mod bytecount;
pub(crate) mod completion;
mod error;
mod getxattr;
mod getxattrs;
mod ioctx;
mod iter_objects;
mod rados;
pub mod read;
mod xattr;

pub use bytecount::ByteCount;
pub(crate) use completion::RadosCompletion;
pub use error::{RadosError, Result};
pub use ioctx::{IoCtx, Namespace, PoolStats};
pub use iter_objects::{Cursor, ListObject, ObjectsIterator, OwnedObject, RawObject, RefObject};
pub use rados::{ClusterStats, FileConfig, Rados, RadosConfig};
pub use xattr::ExtendedAttributes;
