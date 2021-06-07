use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::env;
use std::path::PathBuf;

/// Control the srtool docker container
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// Whether we output json or something for humans
	#[clap(short, long)]
	pub json: bool,

	#[clap(short, long)]
	pub no_cache: bool,

	/// Subcommands are commands passed to `srtool`.
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// This utility allows invoking the srtool with the right parameters and environment variables.
/// See documentations of each command below.
#[derive(Clap)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Build(BuildOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Info(InfoOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Version(VersionOpts),
}

/// Build opts
#[derive(Clap)]
pub struct BuildOpts {
	/// Provide the runtime such as kusama-runtime, polkadot-runtime, etc...
	#[clap(long, short, env = "PACKAGE")]
	pub package: String,

	/// By default, srtool will work in the current folder.
	/// If your project is located in another location, you can pass it here.
	#[clap(index = 1, default_value = ".")]
	pub path: PathBuf,

	/// If your runtime is not in the standard location runtime/<chain_name>
	/// you can pass this args to help srtool find it.
	#[clap(short, long, env = "RUNTIME_DIR")]
	pub runtime_dir: PathBuf,

	/// You may pass options to cargo directly here. WARNING, if you pass
	/// this value, the automatic build options for Kusama and Polkadot will
	/// not be passed and you need to take care of them manually.
	/// In general, you should never use this option unless you HAVE to.
	#[clap(long, env = "BUILD_OPTS")]
	pub build_opts: Option<String>,

	/// Passing this is less involved than passing BUILD_OPTS. It allows
	/// changing the list of default features while keeping the automatic
	/// features detection. This value is useless if BUILD_OPTS is set.
	#[clap(long, env = "DEFAULT_FEATURES")]
	pub default_features: Option<String>,

	/// The default profile to build runtimes is always `release`.
	/// You may override the default with this flag.
	#[clap(long, env = "PROFILE", default_value = "release")]
	pub profile: String,
}

/// Info opts
#[derive(Clap)]
pub struct InfoOpts {
	#[clap(index = 1, default_value = ".")]
	pub path: PathBuf,
}

/// Version opts
#[derive(Clap)]
pub struct VersionOpts;
