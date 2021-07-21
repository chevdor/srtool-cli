//! The runner is effectively a wrapper around docker

use crate::{get_image_digest, run_specs::RunSpecs, version::SrtoolVersion, RuntimeCrate};
use log::{debug, info, trace, warn};
use serde_json::Value;
use std::{
	env, fs,
	path::{Path, PathBuf},
	process::{Command, Stdio},
};

pub struct BuildOpts {
	pub json: bool,
	pub app: bool,
	pub workdir: PathBuf,
}

pub struct Runner;

impl Runner {
	/// Pulls the image
	pub fn pull(image: &str, tag: &str) -> Result<(), String> {
		trace!("pull()");

		debug!("We will be pulling {image}:{tag}", tag = tag, image = image);
		let cmd = format!("docker pull {image}:{tag}", image = image, tag = tag);
		Runner::run(cmd).map(|_| ())
	}

	/// Get version
	pub fn version(image: &str, tag: &str) -> Result<SrtoolVersion, String> {
		let cmd = format!("docker run --name srtool --rm {image}:{tag} version", image = image, tag = tag);
		let version = Runner::run(cmd).unwrap();
		serde_json::from_value::<SrtoolVersion>(version).map_err(|e| e.to_string())
	}

	/// Show infos
	pub fn info(specs: &RunSpecs, workdir: &Path) {
		debug!("specs: '{:#?}'", &specs);

		let cmd = format!(
			"docker run --name srtool --rm \
                -v {dir}:/build \
                -e RUNTIME_DIR={runtime_dir} \
                {image}:{tag} info",
			dir = workdir.display(),
			runtime_dir = specs.runtime_dir.display(),
			image = specs.image,
			tag = specs.tag,
		);
		let _info = Runner::run(cmd).unwrap();
	}

	/// Invoke the build
	pub fn build(specs: &RunSpecs, opts: &BuildOpts) {
		trace!("build()");
		let workdir = fs::canonicalize(&opts.workdir).unwrap();
		debug!("Workdir: {:?}", &workdir);
		println!("We will be using {image}:{tag} for the build", tag = specs.tag, image = specs.image);
		let rtm_crate =
			RuntimeCrate::search_flattened(&workdir, &None, &None, &Some(specs.runtime_dir.to_owned())).unwrap();

		let app = if opts.app { " --app" } else { "" };
		let json = if opts.json { " --json" } else { "" };
		let chain = rtm_crate.package.replace("-runtime", "");
		let default_runtime_dir = format!("runtime/{}", chain);
		let runtime_dir = specs.runtime_dir.to_owned();
		let tmpdir = env::temp_dir().join("cargo");
		let digest = get_image_digest(&specs.image, &specs.tag).unwrap_or_default();
		let cache_mount = if specs.cache_mount {
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

		if let Some(sha256) = &specs.image_sha256 {
			if sha256 == &digest {
				info!("Docker image digest matches: {}", &digest);
			} else {
				warn!("Docker image digests DO NOT match:");
				warn!(" - expected: {} ", &sha256);
				warn!(" - found   : {} ", &digest);
			}
		}

		debug!("cache-mount: '{}'", specs.cache_mount);

		let cmd = format!(
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
			package = rtm_crate.package,
			dir = workdir.display(),
			cache_mount = cache_mount,
			image = specs.image,
			tag = specs.tag,
			runtime_dir = runtime_dir.display(),
			c_build_opts = specs.cargo_build_opts.join(" "),
			default_features = specs.default_features.join(" "),
			profile = specs.profile,
			json = json,
			app = app,
			digest = digest,
		);

		let _digest = Runner::run(cmd).unwrap();
	}

	/// Run the docker command that is passed and retrive its json output
	fn run(cmd: String) -> Result<Value, String> {
		let output = if cfg!(target_os = "windows") {
			Command::new("cmd")
				.args(&["/C", cmd.as_str()])
				.stdout(Stdio::piped())
				.spawn()
				.expect("failed to execute process")
				.wait_with_output()
		} else {
			Command::new("sh")
				.arg("-c")
				.arg(cmd)
				.stdout(Stdio::piped())
				.spawn()
				.expect("failed to execute process")
				.wait_with_output()
		};

		output
			.map(|o| o.stdout)
			.map(|v| String::from_utf8(v).unwrap_or("".into()))
			.map(|s| serde_json::from_str(&s).unwrap_or(Value::Null))
			.map_err(|e| e.to_string())
	}
}

#[cfg(test)]
mod test_runner {
	use super::*;

	#[test]
	fn test_pull() {
		let specs = RunSpecs::default();
		let _ = Runner::pull(&specs.image, &specs.tag);
	}

	#[test]
	fn test_version() {
		let specs = RunSpecs::default();
		let _ = Runner::version(&specs.image, &specs.tag);
	}

	#[test]
	#[ignore = "local data"]
	fn test_info() {
		let specs = RunSpecs::default();
		let workdir = PathBuf::from("/projects/polkadot");
		Runner::info(&specs, &workdir);
	}

	#[test]
	#[ignore = "local data + long running"]
	fn test_build() {
		let specs = RunSpecs::default();
		let workdir = PathBuf::from("/projects/polkadot");
		let opts = BuildOpts { json: true, app: true, workdir };
		Runner::build(&specs, &opts);
	}

	#[test]
	fn test_fake_build_json() {
		let digest = include_str!("../../data/digest_v2_01.json");
		let cmd = format!("docker run --rm -it busybox echo '{}'", digest);

		let res = Runner::run(cmd);
		println!("res = {:?}", res);
	}
}

#[cfg(test)]
mod test_runner_runs {
	use crate::Runner;
	use std::include_str;

	#[test]
	fn test_fake_version_json() {
		let data = include_str!("../../data/version.json");
		let cmd = format!("docker run --rm -it busybox echo '{}'", data);

		let res = Runner::run(cmd).unwrap();
		assert!(res["name"] == "srtool");
		println!("{}", serde_json::to_string_pretty(&res).unwrap());
	}

	#[test]
	fn test_fake_info_json() {
		let data = include_str!("../../data/info.json");
		let cmd = format!("docker run --rm -it busybox echo '{}'", data);

		let res = Runner::run(cmd).unwrap();
		assert!(res["generator"]["name"] == "srtool");

		println!("{}", serde_json::to_string_pretty(&res).unwrap());
	}

	#[test]
	fn test_fake_pull_json() {
		let data = include_str!("../../data/pull.json");
		let cmd = format!("docker run --rm -it busybox echo '{}'", data);

		let res = Runner::run(cmd).unwrap();
		assert!(res.is_null());
	}

	#[test]
	fn test_fake_build_json() {
		let digest = include_str!("../../data/digest_v2_01.json");
		let cmd = format!("docker run --rm -it busybox echo '{}'", digest);

		let res = Runner::run(cmd).unwrap();
		assert!(res["info"]["generator"]["name"] == "srtool");

		println!("{}", serde_json::to_string_pretty(&res).unwrap());
	}
}
