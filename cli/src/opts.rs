use clap::{crate_authors, crate_version, AppSettings, Clap};
use std::env;
use std::path::PathBuf;

/// Control the srtool docker container
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	// /// Whether we output json or something for 'humans'
	// #[clap(short, long)]
	// pub json: bool,
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// This utility allows invoking the srtool with the right parameters and environment variables.
/// See documentations of each command below.
#[derive(Clap)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Build(BuildOpts),
}

/// Build opts
#[derive(Clap)]
pub struct BuildOpts {
	/// Provide the runtime such as kusama-runtime, polkadot-runtime, etc...
	#[clap(long, short)]
	pub package: String,

	#[clap(index = 1, default_value = ".")]
	pub path: PathBuf,
}
