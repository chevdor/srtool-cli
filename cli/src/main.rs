mod opts;
use clap::{crate_version, Parser};
use log::{debug, info};
use opts::*;
use srtool_lib::*;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

mod error;
use error::SrtoolError;

mod container_engine;
use container_engine::ContainerEngine;

fn handle_exit(engine: &ContainerEngine) {
	println!("Killing srtool container, your build was not finished...");
	let cmd = format!("{engine} rm -f srtool");
	let _ = Command::new("sh").arg("-c").arg(cmd).spawn().expect("failed to execute cleaning process").wait();
	println!("Exiting");
	std::process::exit(0);
}

fn main() -> Result<(), SrtoolError> {
	env_logger::init();
	info!("Running srtool-cli v{}", crate_version!());

	let opts: Opts = Opts::parse();
	let image = &opts.image;
	let engine = opts.engine;

	if opts.no_cache {
		let _ = clear_cache();
	}

	ctrlc::set_handler(move || {
		handle_exit(&engine);
	})
	.map_err(|_| SrtoolError::CtrlCSetup)?;

	debug!("Checking what is the latest available tag...");
	const ONE_HOUR: u64 = 60 * 60;

	let tag = get_image_tag(Some(ONE_HOUR)).expect("Issue getting the image tag");

	info!("Using {image}:{tag}");

	let command = match opts.subcmd {
		SubCommand::Pull(_) => {
			println!("Found {tag}, we will be using {image}:{tag} for the build");
			format!("{engine} pull {image}:{tag}")
		}

		SubCommand::Build(build_opts) => {
			println!("Found {tag}, we will be using {image}:{tag} for the build");

			let app = if build_opts.app { " --app" } else { "" };
			let json = if opts.json || build_opts.json { " --json" } else { "" };
			let chain = build_opts.package.replace("-runtime", "");
			let default_runtime_dir = format!("runtime/{chain}");
			let runtime_dir = build_opts.runtime_dir.unwrap_or_else(|| PathBuf::from(&default_runtime_dir));
			let tmpdir = env::temp_dir().join("cargo");
			let digest = get_image_digest(image, &tag).unwrap_or_default();
			let no_cache = if opts.engine == ContainerEngine::Podman { true } else { build_opts.no_cache };
			let cache_mount =
				if !no_cache { format!("-v {tmpdir}:/cargo-home", tmpdir = tmpdir.display()) } else { String::new() };
			let root_opts = if build_opts.root { "-u root" } else { "" };
			let verbose_opts = if build_opts.verbose { "-e VERBOSE=1" } else { "" };

			debug!("engine: '{engine}'");
			debug!("app: '{app}'");
			debug!("json: '{json}'");
			debug!("chain: '{chain}'");
			debug!("default_runtime_dir: '{default_runtime_dir}'");
			debug!("runtime_dir: '{}'", &runtime_dir.display());
			debug!("tmpdir: '{}'", &tmpdir.display());
			debug!("digest: '{digest}'");
			debug!("no-cache: '{}'", no_cache);

			let path = fs::canonicalize(&build_opts.path).unwrap();

			let package = build_opts.package;
			let profile = build_opts.profile;
			let dir = path.display();
			let runtime_dir = runtime_dir.display();
			let c_build_opts = build_opts.build_opts.unwrap_or_else(|| String::from(""));
			let default_features = build_opts.default_features.unwrap_or_else(|| String::from(""));

			format!(
				"{engine} run --name srtool --rm \
				-e PACKAGE={package} \
				-e RUNTIME_DIR={runtime_dir} \
				-e BUILD_OPTS={c_build_opts} \
				-e DEFAULT_FEATURES={default_features} \
				-e PROFILE={profile} \
				-e IMAGE={digest} \
				{verbose_opts} \
				-v {dir}:/build \
				{cache_mount} \
				{root_opts} \
				{image}:{tag} build{app}{json}"
			)
		}

		SubCommand::Info(info_opts) => {
			let path = fs::canonicalize(&info_opts.path).unwrap();
			let chain = info_opts.package.replace("-runtime", "");
			let default_runtime_dir = format!("runtime/{chain}");
			let runtime_dir = info_opts.runtime_dir.unwrap_or_else(|| PathBuf::from(&default_runtime_dir));
			let rtm_dir = runtime_dir.display();

			debug!("chain: '{chain}'");
			debug!("default_runtime_dir: '{default_runtime_dir}'");
			debug!("runtime_dir: '{rtm_dir}'");
			let dir = path.display();

			format!(
				"{engine} run --name srtool --rm \
					-v {dir}:/build \
					-e RUNTIME_DIR={rtm_dir} \
					{image}:{tag} info",
			)
		}

		SubCommand::Version(_) => {
			format!("{engine} run --name srtool --rm {image}:{tag} version")
		}
	};

	debug!("command = {:?}", command);

	let status = if cfg!(target_os = "windows") {
		Command::new("cmd").args(["/C", command.as_str()]).output().expect("failed to execute process").status
	} else {
		match Command::new("sh").arg("-c").arg(command).spawn().expect("failed to execute process").wait_with_output() {
			Ok(result) => result.status,
			_ => std::process::exit(1),
		}
	};

	// Exit with error code of 1 if we don't have an exit code
	std::process::exit(status.code().unwrap_or(1));
}

#[cfg(test)]
mod test {
	use assert_cmd::Command;

	#[test]
	#[ignore = "assert_cmd bug, see https://github.com/assert-rs/assert_cmd/issues/117"]
	fn it_shows_help() {
		let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
		let assert = cmd.arg("--help").assert();
		assert.success().code(0);
	}
}
