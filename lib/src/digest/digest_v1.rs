use semver::Version;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt::Display;
use std::str::FromStr;

//TODO: in V2, in order to NOT break compatibility, some fields are duplicated. That must be reworked. The profile for instance should be in the Context only.

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	T: FromStr,
	T::Err: Display,
	D: Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	T::from_str(&s).map_err(de::Error::custom)
}

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

	#[serde(deserialize_with = "from_str")]
	pub(crate) size: usize,
	pub(crate) prop: String,
	pub(crate) ipfs: String,
	pub(crate) sha256: String,
	pub(crate) wasm: String,
}
