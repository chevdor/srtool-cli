use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::env;
use std::path::PathBuf;

/// Control the srtool docker container
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	/// Choose an alternative image. Beware to choose an image that is
	/// compatible with the original srtool image. Using a random image,
	/// you take the risk to NOT produce exactly the same deterministic
	/// result than srtool.
	#[clap(short, long, default_value = "paritytech/srtool", env = "SRTOOL_IMAGE")]
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
/// Learn more about the srtool image here: https://github.com/paritytech/srtool
#[derive(Clap)]
pub enum SubCommand {
	/// Simply pull the srtool image and do not run anything else
	#[clap(version = crate_version!(), author = crate_authors!())]
	Pull(PullOpts),

	/// Start a new srtool container to build a Substrate based runtime
	#[clap(version = crate_version!(), author = crate_authors!())]
	Build(BuildOpts),

	/// Provide information about the srtool container and your repository
	#[clap(version = crate_version!(), author = crate_authors!())]
	Info(InfoOpts),

	/// Show the versions of the srtool container. Use --version if you want
	/// the version of this executable.
	#[clap(version = crate_version!(), author = crate_authors!())]
	Version(VersionOpts),

	/// Run a new build based on the digest of a previous run in order
	/// to check/verify the result. Such a check may not use the very latest
	/// version of the srtool image but use the same version than used in the
	/// reference run.
	#[clap(version = crate_version!(), author = crate_authors!(), alias = "check")]
	Verify(VerifyOpts),
}

/// Build opts
#[derive(Clap)]
pub struct PullOpts;
// TODO: we may want to let the user specify a repo

/// Build opts
#[derive(Clap)]
pub struct BuildOpts {
	/// If your runtime is not in the standard location runtime/<chain_name>
	/// and/or you use a name different than <chain>-runtime for your runtime crate,
	/// you must pass this value to allow srtool find it.
	#[clap(short, long, env = "RUNTIME_DIR", conflicts_with = "chain", conflicts_with = "package")]
	pub runtime_dir: Option<PathBuf>,

	/// If your runtime is in the standard location, you can simply pass its name here.
	/// This is assuming your runtime crate is called <chain>-runtime and located under
	/// `runtime/<chain>`.
	#[clap(short, long, env = "CHAIN", conflicts_with = "package")]
	pub chain: Option<String>,

	/// Passing the `chain` argument is probably easier but this option is left
	/// for compatibility reason. You may pass this value if and it formatted as
	/// <chain>-runtime and your runtime crate is under `runtime/<chain>`. For other
	/// case, please pass `runtime-dir` to disambiguate.
	#[clap(long, short, env = "PACKAGE")]
	pub package: Option<String>,

	/// Enable json output, same than the global --json option
	#[clap(long, short)]
	pub json: bool,

	/// Enable the "app" mode which is a mix of json output and
	/// outputting progress during the build. This flag is recommended for CI.
	/// the json output will be provided as a single line at the end in compact mode.
	#[clap(long, short)]
	pub app: bool,

	/// By default, srtool will work in the current folder.
	/// If your project is located in another location, you can pass it here.
	#[clap(index = 1, default_value = ".")]
	pub workdir: PathBuf,

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
	/// You may override the default with this flag should you need it,
	/// which is btw very unlikely.
	#[clap(long, env = "PROFILE", default_value = "release")]
	pub profile: String,

	/// Passing this flag allows completely disabling caching.
	/// As a result, no cargo-home will be mounted to the srtool image.
	/// There is no known issue with having the cache ON, this is why it is the default.
	#[clap(long)]
	pub no_cache: bool,
}

/// Info opts
#[derive(Clap)]
pub struct InfoOpts {
	/// By default, srtool will work in the current folder.
	/// If your project is located in another location, you can pass it here.
	#[clap(index = 1, default_value = ".")]
	pub workdir: PathBuf,

	/// Provide the runtime such as kusama-runtime, polkadot-runtime, etc...
	#[clap(long, short, env = "CHAIN", conflicts_with = "package", conflicts_with = "runtime-dir")]
	pub chain: Option<String>,

	/// Provide the runtime such as kusama-runtime, polkadot-runtime, etc...
	#[clap(long, short, env = "PACKAGE", conflicts_with = "runtime-dir")]
	pub package: Option<String>,

	/// If your runtime is not in the standard location runtime/<chain_name>
	/// you can pass this args to help srtool find it.
	#[clap(short, long, env = "RUNTIME_DIR")]
	pub runtime_dir: Option<PathBuf>,
}

/// Version opts
#[derive(Clap)]
pub struct VersionOpts;

/// Verify opts
#[derive(Clap)]
pub struct VerifyOpts {
	/// The path of the srtool digest (json) where most of the settings
	/// will be fetched to reproduce the exact same build.
	#[clap(long, short, default_value = "digest.json", required = true)]
	pub digest: PathBuf,

	/// By default, srtool will work in the current folder.
	/// If your project is located in another location, you can pass it here.
	#[clap(index = 1, default_value = ".")]
	pub workdir: PathBuf,
}
