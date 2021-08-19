use crate::rustc_version::RustcVersion;
use semver::Version;
use serde::{Deserialize, Serialize};

/// A structure describing the output the info command
#[derive(Debug, Serialize, Deserialize)]
pub struct SrtoolVersion {
	name: String,
	version: Version,

	// #[serde(deserialize_with= "RustcVersion::from_str")]
	rustc: RustcVersion,
}

#[cfg(test)]
mod test_version {
	use super::*;
	use std::include_str;

	#[test]
	fn test_deserialize_from_str() {
		let txt = include_str!("../../data/info.json");
		let v: SrtoolVersion = serde_json::from_str(txt).unwrap();
		println!("v = {:?}", v);
	}

	#[test]
	fn test_deserialize_from_json() {
		let txt = include_str!("../../data/info.json");
		let json = serde_json::from_str(txt).unwrap();
		let v: SrtoolVersion = serde_json::from_value(json).unwrap();
		println!("v = {:?}", v);
	}
}
