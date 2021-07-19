//! Ideally, we don't need this struct and everything is available under the `Context`.
//! However, V2 does not contain enough in the Context and has some of the information under
//! the `Info` key. So we bring everything together as `RunSpecs`.

pub struct RunSpecs {
	/// Name of the crate of the runtime
	pub(crate) package: String,

	/// Path to the runtime crate relative to the root of the repository
	pub(crate) runtime_dir: String,

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
