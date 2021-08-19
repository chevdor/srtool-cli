use super::digest_source::DigestSource;
use super::versionned_digest::Digest;
use serde_json::Result;
use serde_json::Value;
type Json = Value;

pub struct DigestJson {}

impl DigestSource<Json> for DigestJson {
	fn load(src: Json) -> Result<Digest> {
		let digest: Digest = serde_json::from_str(&src.to_string())?;
		Ok(digest)
	}
}

#[cfg(test)]
mod test_super {
	use super::*;
	use crate::{
		digest::digest_v2,
		samples::{SAMPLE_V1, SAMPLE_V2},
	};
	use serde_json::json;

	#[test]
	fn test_v1() {
		let v1: Value = serde_json::from_str(SAMPLE_V1).unwrap();
		let digest = DigestJson::load(json!({ "V1": v1 })).unwrap();

		match digest {
			Digest::V1(v1) => assert!(v1.src == "git"),
			Digest::V2(v2) => assert!(v2.info.src == digest_v2::Source::Git),
		}
	}

	#[test]
	fn test_v2() {
		let v2: Value = serde_json::from_str(SAMPLE_V2).unwrap();
		let digest = DigestJson::load(json!({ "V2": v2 })).unwrap();

		match digest {
			Digest::V1(v1) => assert!(v1.src == "git"),
			Digest::V2(v2) => assert!(v2.info.src == digest_v2::Source::Git),
		}
	}
}
