use std::path::PathBuf;

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Source {
	#[serde(alias = "git")]
	Git,
	Archive,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Generator {
	pub name: String,
	pub version: Version,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GitInfo {
	pub commit: String,
	pub tag: String,
	pub branch: String,
}

//TODO: in V2, in order to NOT break compatibility, some fields are duplicated. That must be reworked. The profile for instance should be in the Context only.

/// The difference between Info and Context is that the content
/// of Info is (should be...) derivated from Context. In other words, once we have
/// the Context, we can extract all the Info.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
	/// Information about the tooling used for the build.
	pub(crate) generator: Generator,

	/// Whether the build from an Archive or from a Git repo.
	pub(crate) src: Source,

	/// The version of the crate/package to build
	pub(crate) version: Version,

	/// Optionnal Git information if the src was Git
	pub(crate) git: Option<GitInfo>,

	/// Rust compiler version
	pub(crate) rustc: String,

	/// Package
	pub(crate) pkg: String,

	/// Profile. Always 'release'.
	pub(crate) profile: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerContext {
	pub image: String,
	pub tag: String,
	pub digest: Option<String>,
}

/// This struct describes all the information required
/// by srtool to build a runtime.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Context {
	pub(crate) docker: DockerContext,
	pub(crate) runtime_dir: PathBuf,
	pub(crate) package: String,
	pub(crate) profile: String,
}

/// A srtool digest. The schema of the data srtool produces may
/// change over time. This struct can load all version and make
/// the common and relevant data available.
// TODO: This is a piece that should be shared with/coming from srtool-cargo.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct V2 {
	pub(crate) info: Info,
	pub(crate) context: Context,
}

impl V2 {
	/// The `full_tag` is made of <rustc_version>-<srtool_build_version>.
	/// While using only <rustc_version> will produce the same artifacts, we have no insurance
	/// that another version of the srtool build provides the same output.
	pub fn get_full_tag(&self) -> String {
		format!("{}-{}", self.context.docker.tag.to_owned(), self.info.generator.version)
	}
}
