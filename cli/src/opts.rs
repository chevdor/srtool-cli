use clap::{crate_authors, crate_version, Parser, Subcommand};
use std::{env, process::Command};
use std::path::PathBuf;

use crate::{ContainerEngine, Error};

/// Control the srtool docker container
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Opts {
	/// Choose an alternative image. Beware to choose an image that is
	/// compatible with the original srtool image. Using a random image,
	/// you take the risk to NOT produce exactly the same deterministic
	/// result as srtool.
	#[clap(short, long, default_value = "docker.io/paritytech/srtool", global = true)]
	pub image: String,

	/// This option is DEPRECATED and has no effect
	#[clap(short, long)]
	pub json: bool,

	/// Do not use the local cached tag value
	#[clap(short, long)]
	pub no_cache: bool,

	#[clap(short, long, global = true, default_value = "auto")]
	pub engine: String,

	/// Subcommands are commands passed to `srtool`.
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

impl Opts {
	pub fn get_container_engine(&self) -> Result<ContainerEngine, Error> {
		match self.engine.as_str() {
			"auto" => Self::detect_container_engine(),
			"podman" => Ok(ContainerEngine::Podman),
			"docker" => {
				println!("WARNING: You are using docker. We recommend using podman instead.");
				Ok(ContainerEngine::Docker)
			},
			_ => Err(Error::UnknownContainerEngine),
		}
	}

	fn detect_container_engine() -> Result<ContainerEngine, Error> {
		let podman_output: Option<std::process::Output> = Command::new("podman").arg("--version").output().ok();
		if let Some(podman) = podman_output {
			let podman = String::from_utf8_lossy(&podman.stdout);
			if podman.to_lowercase().contains("podman") {
				return Ok(ContainerEngine::Podman);
			} else if podman.contains("docker") {
				println!("WARNING: You have podman symlinked to docker. This is strange.");
				return Ok(ContainerEngine::Docker);
			}
		}
		
		let docker_output = Command::new("docker").arg("--version").output().ok();
		if let Some(docker) = docker_output {
			let docker = String::from_utf8_lossy(&docker.stdout);
			if docker.to_lowercase().contains("docker") {
				println!("WARNING: You are using docker. We recommend using podman instead.");
				return Ok(ContainerEngine::Docker);
			} else if docker.contains("podman") {
				return Ok(ContainerEngine::Podman);
			}
		}

		return Err(Error::UnknownContainerEngine);

	}
}

/// This utility helps starting a container from the srtool Docker image.
/// It passes the right parameters and environment variables to the container.
/// Learn more about the srtool image here: https://github.com/paritytech/srtool
#[derive(Subcommand)]
pub enum SubCommand {
	/// Simply pull the srtool image and do not run anything else
	#[clap(version = crate_version!(), author = crate_authors!())]
	Pull(PullOpts),

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
#[derive(Parser)]
pub struct PullOpts;

/// Build opts
#[derive(Parser)]
pub struct BuildOpts {
	/// Provide the runtime such as kusama-runtime, polkadot-runtime, etc...
	#[clap(long, short, env = "PACKAGE")]
	pub package: String,

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

	/// Passing this flag allows completely disabling caching.
	/// As a result, no cargo-home will be mounted to the srtool image.
	/// There is no known issue with having the cache ON, this is why it is the default.
	#[clap(long)]
	pub no_cache: bool,

	/// Run container image as root, this helps on Linux based systems.
	#[clap(long)]
	pub root: bool,

	/// Run the build script using the verbose option
	#[clap(long)]
	pub verbose: bool,
}

/// Info opts
#[derive(Parser)]
pub struct InfoOpts {
	/// By default, srtool will work in the current folder.
	/// If your project is located in another location, you can pass it here.
	#[clap(index = 1, default_value = ".")]
	pub path: PathBuf,

	/// Provide the runtime such as kusama-runtime, polkadot-runtime, etc...
	#[clap(long, short, env = "PACKAGE")]
	pub package: String,

	/// If your runtime is not in the standard location runtime/<chain_name>
	/// you can pass this args to help srtool find it.
	#[clap(short, long, env = "RUNTIME_DIR")]
	pub runtime_dir: Option<PathBuf>,
}

/// Version opts
#[derive(Parser)]
pub struct VersionOpts;
