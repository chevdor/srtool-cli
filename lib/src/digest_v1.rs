use semver::Version;
use serde::{Deserialize, Serialize};

//TODO: in V2, in order to NOT break compatibility, some fields are duplicated. That must be reworked. The profile for instance should be in the Context only.

/// A srtool digest. The schema of the data srtool produces may
/// change over time. This struct can load all version and make
/// the common and relevant data available.
// TODO: This is a piece that should be shared with/coming from srtool-cargo.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct V1 {
	pub(crate) gen: String,
	pub(crate) src: String,
	pub(crate) version: Version,
	pub(crate) commit: String,
	pub(crate) tag: String,
	pub(crate) branch: String,
	pub(crate) rustc: String,
	pub(crate) pkg: String,
	pub(crate) tmsp: String,
	pub(crate) size: uisze,
	pub(crate) prop: String,
	pub(crate) ipfs: String,
	pub(crate) sha256: String,
	pub(crate) wasm: String,
}
