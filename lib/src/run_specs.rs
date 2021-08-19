//! Ideally, we don't need this struct and everything is available under the `Context`.
//! However, V2 does not contain enough in the Context and has some of the information under
//! the `Info` key. So we bring everything together as `RunSpecs`.

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RunSpecs {
	/// Path to the runtime crate relative to the root of the repository
	pub runtime_dir: PathBuf,

	/// Usually `release`
	pub profile: String,

	/// The docker image, ie: paritytech/srtool
	pub image: String,

	/// The digest of the docker image
	pub image_sha256: Option<String>,
	pub cargo_build_opts: Vec<String>,
	pub default_features: Vec<String>,

	pub tag: String,
	pub cache_mount: bool,
}

impl RunSpecs {
	pub fn new(
		runtime_dir: &Path,
		profile: &str,
		image: &str,
		tag: &str,
		image_sha256: Option<String>,
		cache_mount: bool,
	) -> Self {
		Self {
			runtime_dir: runtime_dir.to_owned(),
			profile: profile.to_string(),
			image: image.to_string(),
			tag: tag.to_string(),
			image_sha256,
			cargo_build_opts: Vec::new(), // TODO,
			default_features: Vec::new(), // TODO
			cache_mount,
		}
	}
}

#[cfg(test)]
/// Default is only used as convenience for the tests.
impl Default for RunSpecs {
	fn default() -> Self {
		Self {
			runtime_dir: PathBuf::from("runtime/polkadot"),
			profile: "release".to_string(),
			image: "paritytech/srtool".to_string(),
			image_sha256: Some(String::from("sha256:31a302da3198ac5d9fe0beb5cb4b456552e8745544172dffa244c439750c0133")),
			cargo_build_opts: vec![],
			default_features: vec![],
			tag: "1.53.0-0.9.15".to_string(),
			cache_mount: true,
		}
	}
}
