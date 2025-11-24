pub mod embed;
pub mod error;
pub mod memory;
pub mod storage;
pub mod vector_store;
pub mod wasm;

#[cfg(feature = "fastembed")]
#[cfg_attr(docsrs, doc(cfg(feature = "fastembed")))]
pub mod fastembed;

use error::Error;
