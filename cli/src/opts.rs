use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::env;
use std::path::PathBuf;

/// Control the srtool docker container
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// Chose an alternative image. Beware to chose an image that is
	/// compatible with the original srtool image. Using a random image,
	/// you take the risk to NOT produce exactly the same deterministic
	/// result than srtool.
	#[clap(short, long, default_value = "chevdor/srtool")]
	pub image: String,

	/// This option is DEPRECATED and has no effect
	#[clap(short, long)]
	pub json: bool,

	/// Do not use the local cached tag value
	#[clap(short, long)]
	pub no_cache: bool,

	/// Subcommands are commands passed to `srtool`.
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// This utility helps starting a container from the srtool Docker image.
/// It passes the right parameters and environment variables to the container.
/// Learn more about the srtool image here: https://gitlab.com/chevdor/srtool
#[derive(Clap)]
pub enum SubCommand {
	/// Start a new srtool container to build your runtime
	#[clap(version = crate_version!(), author = crate_authors!())]
	Build(BuildOpts),

	/// Provide information about the srtool container and your repo
	#[clap(version = crate_version!(), author = crate_authors!())]
	Info(InfoOpts),

	/// Show the versions of the srtool container. Use --version if you want
	/// the version of this executable.
	#[clap(version = crate_version!(), author = crate_authors!())]
	Version(VersionOpts),
}

/// Build opts
#[derive(Clap)]
pub struct BuildOpts {
	/// Provide the runtime such as kusama-runtime, polkadot-runtime, etc...
	#[clap(long, short, env = "PACKAGE")]
	pub package: String,

	/// Enable json output, same than the global --json option
	#[clap(long, short)]
	pub json: bool,

	/// Enable the "app" mode which is a mix of json output and
	/// outputing progress during the build. This flag is recommended for CI.
	/// the json output will be provided as a single line at the end in compact mode.
	#[clap(long, short)]
	pub app: bool,

	/// By default, srtool will work in the current folder.
	/// If your project is located in another location, you can pass it here.
	#[clap(index = 1, default_value = ".")]
	pub path: PathBuf,

	/// If your runtime is not in the standard location runtime/<chain_name>
	/// you can pass this args to help srtool find it.
	#[clap(short, long, env = "RUNTIME_DIR")]
	pub runtime_dir: Option<PathBuf>,

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
