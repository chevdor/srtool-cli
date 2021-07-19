//! The srtool digests format has evolved over time. The first versions were rather flat
//! and no old schema has been written here, at least not yet. That means the old digests
//! are currently not supported. The first schema that is supported is called V2 and matches the
//! output of srtool v0.9.15. The latest V2 is compatible with the old format and thus contains
//! duplication. V10 will be a chance for a good cleanup.
//! Especially, the split between content in Info and Context should be improved.

// TODO: The code for the srtool digest needs to be moved under srtool-cargo once published.

use std::str::FromStr;

use crate::{digest_v2::V2, run_specs::RunSpecs};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Digest {
	V2(V2),
}

impl Digest {
	pub fn get_run_specs(&self) -> Result<RunSpecs, String> {
		match self {
			Digest::V2(v2) => Ok(RunSpecs {
				package: v2.context.package.to_owned(),
				runtime_dir: v2.context.runtime_dir.to_owned(),
				profile: v2.context.profile.to_owned(),
				image: v2.context.docker.image.to_owned(),
				image_sha256: String::new(),  // TODO
				cargo_build_opts: Vec::new(), // TODO
				default_features: Vec::new(), // TODO
				tag: v2.context.docker.full_tag.to_owned(),
				cache_mount: false, // TODO
			}),
		}
	}

	pub fn get_version(json: Value) -> Option<Version> {
		let version_v1 = &json["gen"].as_str().unwrap_or_default().split('v').nth(1);
		let version_v2 = &json["info"]["generator"]["version"].as_str();

		if let Some(v) = version_v2 {
			return Some(Version::from_str(v).unwrap());
		}

		if let Some(v) = version_v1 {
			return Some(Version::from_str(v).unwrap());
		}

		None
	}
}

impl From<Value> for Digest {
	fn from(v: Value) -> Self {
		if !v.is_object() {
			panic!("Invalid digest, it should be a JSON Object");
		}

		let version = Digest::get_version(v);

		match version {
			// TODO: exact version to double check, not sure when the new format was introduced, 0.9.13 or 0.9.14
			Some(v) if VersionReq::parse("<=0.9.13").unwrap().matches(&v) => {
				// V1
				todo!()
			}
			Some(v) if VersionReq::parse(">0.9.14 <=0.9.15").unwrap().matches(&v) => {
				// V2
				todo!()
			}
			Some(v) => panic!("Version {} is not supported", v),
			None => unreachable!(),
		}
	}
}

#[cfg(test)]
mod test_digest {
	use super::*;
	use crate::samples::*;

	#[test]
	fn test_version_from_json_v1() {
		let v1: Value = serde_json::from_str(SAMPLE_V1).unwrap();
		assert_eq!(Digest::get_version(v1), Some(Version::from_str("0.9.14").unwrap()));
	}

	#[test]
	fn test_version_from_json_v2() {
		let v2: Value = serde_json::from_str(SAMPLE_V2).unwrap();
		assert_eq!(Digest::get_version(v2), Some(Version::from_str("0.9.15").unwrap()));
	}

	#[test]
	fn test_version_from_json_v3() {
		let v3: Value = serde_json::from_str(SAMPLE_V3).unwrap();
		assert_eq!(Digest::get_version(v3), Some(Version::from_str("0.9.17").unwrap()));
	}

	#[test]
	fn test_version_from_json_unknown() {
		let v4: Value = serde_json::from_str(SAMPLE_V4).unwrap();
		assert_eq!(Digest::get_version(v4), None);
	}
}
