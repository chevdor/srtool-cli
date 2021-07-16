use serde_json::Result;
use serde_json::Value;

use crate::{digest::Digest, digest_source::DigestSource};
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
	use crate::digest_v9;
	use serde_json::json;

	#[test]
	fn test_() {
		let json = json!({
			"V9": {
				"context": {
					"docker": {
						"image": "paritytech/srtool",
						"full_tag": "1.53.0-0.9.15",
					},
					"runtime_dir": "runtime/polkadot",
					"package": "polkadot-runtime",
					"profile": "release",
				},
				"source": "git",
				"info": {

					"generator": {
						"name": "srtool",
						"version": "0.9.15",
					},
					"src": "git",
					"version": "0.9.7",
					"git": {
						"commit": "5d35bac7408a4cb12a578764217d06f3920b36aa",
						"tag": "v0.9.7-rc3",
						"branch": "heads/v0.9.7-rc3",
					},
					"rustc": "rustc 1.53.0 (53cb7b09b 2021-06-17)",
					"pkg": "polkadot-runtime",
					"profile": "release",
				},
			}
		});
		let digest = DigestJson::load(json).unwrap();

		match digest {
			Digest::V9(v9) => assert!(v9.source == digest_v9::Source::Git),
		}
	}
}
