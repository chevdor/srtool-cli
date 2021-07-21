mod opts;
use clap::{crate_name, crate_version, Clap};
use log::{debug, info};
use opts::*;
use serde_json::json;
use srtool_lib::*;
use std::fs::File;
use std::io::BufReader;
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
	info!("Running {} v{}", crate_name!(), crate_version!());

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

	match opts.subcmd {
		SubCommand::Pull(_) => Runner::pull(&image, &tag),
		SubCommand::Version(_) => Runner::version(&image, &tag),

		SubCommand::Info(info_opts) => {
			let workdir = fs::canonicalize(&info_opts.workdir).unwrap();
			let rtm_crate =
				RuntimeCrate::search_flattened(&workdir, &info_opts.package, &info_opts.chain, &info_opts.runtime_dir)
					.unwrap();

			let specs = RunSpecs::new(&rtm_crate.runtime_dir, "release", &image, &tag, None, false);
			Runner::info(&specs, &rtm_crate.workdir);
		}

		SubCommand::Build(build_opts) => {
			let workdir = fs::canonicalize(&build_opts.workdir).unwrap();

			let rtm_crate = RuntimeCrate::search_flattened(
				&workdir,
				&build_opts.package,
				&build_opts.chain,
				&build_opts.runtime_dir,
			)
			.unwrap();

			let specs =
				RunSpecs::new(&rtm_crate.runtime_dir, &build_opts.profile, &image, &tag, None, !build_opts.no_cache);
			let opts = srtool_lib::BuildOpts { json: build_opts.json, app: build_opts.app, workdir };
			Runner::build(&specs, &opts);
		}

		SubCommand::Verify(verify_opts) => {
			// let workdir = fs::canonicalize(&verify_opts.workdir).unwrap();

			debug!("Digest from: {:?}", verify_opts.digest);
			let file = File::open(verify_opts.digest).unwrap();
			let reader = BufReader::new(file);
			let content: V2 = serde_json::from_reader(reader).unwrap();
			let digest_json = json!({ "V2": content });
			let digest = DigestJson::load(json!(digest_json)).unwrap();
			debug!("digest = {:#?}", digest);
			let specs = digest.get_run_specs().unwrap();
			debug!("specs = {:#?}", specs);
			let build_opts = srtool_lib::BuildOpts { json: true, app: true, workdir: "/projects/polkadot".into() };

			// let rtm_crate = RuntimeCrate::search_flattened(&workdir, &None, &None, &Some(specs.runtime_dir)).unwrap();

			Runner::build(&specs, &build_opts);
			todo!()
		}
	};
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
