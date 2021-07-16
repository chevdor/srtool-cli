//! The srtool digests format has evolved over time. The first versions were rather flat
//! and no old schema has been written here, at least not yet. That means the old digests 
//! are currently not supported. The first schema that is supported is called V9 and matches the 
//! output of srtool v0.9.15. The latest V9 is compatible with the old format and thus contains
//! duplication. V10 will be a chance for a good cleanup.
//! Especially, the split between content in Info and Context should be improved.

use crate::digest_v9::V9;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Digest {
    V9(V9),
}
