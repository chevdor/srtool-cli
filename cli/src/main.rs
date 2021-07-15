mod opts;
use clap::{crate_version, Clap};
use log::{debug, info};
use opts::*;
use srtool_lib::*;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn handle_exit() {
	println!("Killing srtool container, your build was not finished...");
	let cmd = "docker rm -f srtool".to_string();
	let _ = Command::new("sh").arg("-c").arg(cmd).spawn().expect("failed to execute cleaning process").wait();
	println!("Exiting");
	std::process::exit(0);
}

fn main() {
	env_logger::init();
	info!("Running srtool-cli v{}", crate_version!());

	let opts: Opts = Opts::parse();
	let image = opts.image;

	if opts.no_cache {
		clear_cache();
	}

	ctrlc::set_handler(move || {
		handle_exit();
	})
	.expect("Error setting Ctrl-C handler");

	debug!("Checking what is the latest available tag...");
	const ONE_HOUR: u64 = 60 * 60;

	let tag = get_image_tag(Some(ONE_HOUR)).expect("Issue getting the image tag");

	info!("Using {}:{}", image, tag);

	let command = match opts.subcmd {
		SubCommand::Pull(_) => {
			println!("Found {tag}, we will be using {image}:{tag} for the build", tag = tag, image = image);
			format!("docker pull {image}:{tag}", image = image, tag = tag,)
		}

		SubCommand::Build(build_opts) => {
			println!("Found {tag}, we will be using {image}:{tag} for the build", tag = tag, image = image);

			let app = if build_opts.app { " --app" } else { "" };
			let json = if opts.json || build_opts.json { " --json" } else { "" };
			let chain = build_opts.package.replace("-runtime", "");
			let default_runtime_dir = format!("runtime/{}", chain);
			let runtime_dir = build_opts.runtime_dir.unwrap_or_else(|| PathBuf::from(&default_runtime_dir));
			let tmpdir = env::temp_dir().join("cargo");
			let digest = get_image_digest(&image, &tag).unwrap_or_default();
			let cache_mount = if !build_opts.no_cache {
				format!("-v {tmpdir}:/cargo-home", tmpdir = tmpdir.display())
			} else {
				String::new()
			};

			debug!("app: '{}'", &app);
			debug!("json: '{}'", &json);
			debug!("chain: '{}'", &chain);
			debug!("default_runtime_dir: '{}'", &default_runtime_dir);
			debug!("runtime_dir: '{}'", &runtime_dir.display());
			debug!("tmpdir: '{}'", &tmpdir.display());
			debug!("digest: '{}'", &digest);
			debug!("no-cache: '{}'", build_opts.no_cache);

			let path = fs::canonicalize(&build_opts.path).unwrap();

			format!(
				"docker run --name srtool --rm \
				-e PACKAGE={package} \
				-e RUNTIME_DIR={runtime_dir} \
				-e BUILD_OPTS={c_build_opts} \
				-e DEFAULT_FEATURES={default_features} \
				-e PROFILE={profile} \
				-e IMAGE={digest} \
				-v {dir}:/build \
				{cache_mount} \
				{image}:{tag} build{app}{json}",
				package = build_opts.package,
				dir = path.display(),
				cache_mount = cache_mount,
				image = image,
				tag = tag,
				runtime_dir = runtime_dir.display(),
				c_build_opts = build_opts.build_opts.unwrap_or_else(|| String::from("")),
				default_features = build_opts.default_features.unwrap_or_else(|| String::from("")),
				profile = build_opts.profile,
				json = json,
				app = app,
				digest = digest,
			)
		}
		SubCommand::Info(info_opts) => {
			let path = fs::canonicalize(&info_opts.path).unwrap();

			format!(
				"docker run --name srtool --rm  -v {dir}:/build {image}:{tag} info",
				dir = path.display(),
				image = image,
				tag = tag,
			)
		}

		SubCommand::Version(_) => {
			format!("docker run --name srtool --rm {image}:{tag} version", image = image, tag = tag,)
		}
	};

	debug!("command = {:?}", command);

	if cfg!(target_os = "windows") {
		Command::new("cmd").args(&["/C", command.as_str()]).output().expect("failed to execute process");
	} else {
		let _ =
			Command::new("sh").arg("-c").arg(command).spawn().expect("failed to execute process").wait_with_output();
	}
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
