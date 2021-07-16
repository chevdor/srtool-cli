use semver::Version;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Source {
    #[serde(alias = "git")]
    Git,
    Archive,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Generator {
    name: String,
    version: Version,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GitInfo {
    commit: String,
    tag: String,
    branch: String,
}

//TODO: in V9, in order to NOT break compatibility, some fields are duplicated. That must be reworked. The profile for instance should be in the Context only.

/// The difference between Info and Context is that the content
/// of Info is (should be...) derivated from Context. In other words, once we have
/// the Context, we can extract all the Info.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    /// Information about the tooling used for the build.
    generator: Generator,

    /// Whether the build from an Archive or from a Git repo.
    src: Source,

    /// The version of the crate/package to build
    version: Version,

    /// Optionnal Git information if the src was Git
    git: Option<GitInfo>,

    /// Rust compiler version
    rustc: String,

    /// Package
    pkg: String,

    /// Profile. Always 'release'.
    profile: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DockerContext {
    pub image: String,
    pub full_tag: String,
}

/// This struct describes all the information required
/// by srtool to build a runtime.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Context {
    pub docker: DockerContext,
    pub runtime_dir: String,
    pub package: String,
    pub profile: String,
}

/// A srtool digest. The schema of the data srtool produces may
/// change over time. This struct can load all version and make
/// the common and relevant data available.
// TODO: This is a piece that should be shared with/coming from srtool-cargo.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct V9 {
    pub info: Info,
    pub context: Context,
    pub source: Source,

}
