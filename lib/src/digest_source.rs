use crate::digest::Digest;
use serde_json::Result;

/// This trait describes digest sources. Those could be
/// File, Http, Ipfs, etc....
pub trait DigestSource<S> {
    fn load(source: S) -> Result<Digest>;
}
