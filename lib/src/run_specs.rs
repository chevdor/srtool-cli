//! Ideally, we don't need this struct and everything is available under the `Context`.
//! However, V2 does not contain enough in the Context and has some of the information under
//! the `Info` key. So we bring everything together as `RunSpecs`.

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct RunSpecs {
	/// Name of the crate of the runtime
	pub(crate) package: String,

	/// Path to the runtime crate relative to the root of the repository
	pub(crate) runtime_dir: PathBuf,

	/// Usually `release`
	pub(crate) profile: String,

	/// The docker image, ie: paritytech/srtool
	pub(crate) image: String,

	/// The digest of the docker image
	pub(crate) image_sha256: String,
	pub(crate) cargo_build_opts: Vec<String>,
	pub(crate) default_features: Vec<String>,

	pub(crate) tag: String,
	pub(crate) cache_mount: bool,
}

impl RunSpecs {
	pub fn new(package: &str, runtime_dir: &Path, profile: &str, image: &str, tag: &str) -> Self {
		Self {
			package: package.to_string(),
			runtime_dir: runtime_dir.to_owned(),
			profile: profile.to_string(),
			image: image.to_string(),
			tag: tag.to_string(),
			image_sha256: String::new(),  // TODO
			cargo_build_opts: Vec::new(), // TODO,
			default_features: Vec::new(), // TODO
			cache_mount: true,            // TODO
		}
	}
}

#[cfg(test)]
/// Default is only used as convenience for the tests.
impl Default for RunSpecs {
	fn default() -> Self {
		Self {
			package: "polkadot-runtime".to_string(),
			runtime_dir: PathBuf::from("runtime/polkadot"),
			profile: "release".to_string(),
			image: "paritytech/srtool".to_string(),
			image_sha256: String::new(),
			cargo_build_opts: vec![],
			default_features: vec![],
			tag: "1.53.0-0.9.15".to_string(),
			cache_mount: true,
		}
	}
}
