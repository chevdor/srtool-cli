//! The runner is effectively a wrapper around docker

use crate::{get_image_digest, run_specs::RunSpecs};
use log::debug;
use std::{
	env, fs,
	path::{Path, PathBuf},
	process::Command,
};

pub struct BuildOpts {
	pub json: bool,
	pub app: bool,
	pub workdir: PathBuf,
}

pub struct Runner;

impl Runner {
	/// Pulls the image
	pub fn pull(image: &str, tag: &str) {
		debug!("Found {tag}, we will be using {image}:{tag} for the build", tag = tag, image = image);
		let cmd = format!("docker pull {image}:{tag}", image = image, tag = tag);
		Runner::run(cmd);
	}

	/// Invoke the build
	pub fn build(specs: &RunSpecs, opts: &BuildOpts) {
		println!("Found {tag}, we will be using {image}:{tag} for the build", tag = specs.tag, image = specs.image);

		let app = if opts.app { " --app" } else { "" };
		let json = if opts.json { " --json" } else { "" };
		let chain = specs.package.replace("-runtime", "");
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
		debug!("cache-mount: '{}'", specs.cache_mount);

		let path = fs::canonicalize(&opts.workdir).unwrap();

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
			package = specs.package,
			dir = path.display(),
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
		Runner::run(cmd);
	}

	/// Show infos
	pub fn info(specs: &RunSpecs, workdir: &Path) {
		// let path = fs::canonicalize(&workdir).unwrap();
		let chain = specs.package.replace("-runtime", "");
		// let default_runtime_dir = format!("runtime/{}", chain);

		debug!("specs: '{:#?}'", &specs);
		debug!("chain: '{}'", &chain);

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
		Runner::run(cmd);
	}

	/// Get version
	pub fn version(image: &str, tag: &str) {
		let cmd = format!("docker run --name srtool --rm {image}:{tag} version", image = image, tag = tag);
		Runner::run(cmd);
	}

	/// Run the docker command that is passed
	fn run(cmd: String) {
		if cfg!(target_os = "windows") {
			Command::new("cmd").args(&["/C", cmd.as_str()]).output().expect("failed to execute process");
		} else {
			let _ =
				Command::new("sh").arg("-c").arg(cmd).spawn().expect("failed to execute process").wait_with_output();
		}
	}
}

#[cfg(test)]
mod test_runner {
	use super::*;

	#[test]
	fn test_pull() {
		let specs = RunSpecs::default();
		Runner::pull(&specs.image, &specs.tag);
	}

	#[test]
	fn test_version() {
		let specs = RunSpecs::default();
		Runner::version(&specs.image, &specs.tag);
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
}
