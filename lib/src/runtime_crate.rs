use std::{fs, path::PathBuf};
use toml::Value;

use std::error::Error;

/// This sctruct holds the information required to know which
/// runtime to build.
#[derive(Debug)]
pub struct RuntimeCrate {
	workdir: PathBuf,
	runtime_dir: PathBuf,
	package: String,
	chain: String,
}

#[derive(Debug)]
pub enum RuntimeCrateSearchOption {
	RuntimeDir(String),
	Package(String),
	ChainName(String),
}

/// This is the data to provide to search for the runtime.
/// Note that the only reliable way is to pass workdir + runtime_dir.
/// All other options and combinations have more chances to fail.
#[derive(Debug)]
pub struct RuntimeCrateSearchInfo {
	pub workdir: PathBuf,
	pub options: Option<RuntimeCrateSearchOption>,
}

impl RuntimeCrate {
	/// This function helps find the runtime crate based on *some* information.
	/// The result will be Ok if and only if the search critera lead to one single
	/// result. In any other cases, it will return an error.
	pub fn search(input: &RuntimeCrateSearchInfo) -> Result<RuntimeCrate, Box<dyn Error>> {
		match &input.options {
			Some(opts) => match opts {
				// This is the less fuzzy option
				RuntimeCrateSearchOption::RuntimeDir(runtime_dir) => {
					let runtime_dir = PathBuf::from(runtime_dir);
					let cargo_toml = input.workdir.join(&runtime_dir).join("Cargo.toml");
					let toml_content: Value = fs::read_to_string(cargo_toml)?.parse()?;
					let package = toml_content["package"]["name"].as_str().expect("Failed getting the package name");
					let chain = package.replace("-runtime", "");
					Ok(RuntimeCrate {
						workdir: input.workdir.to_owned(),
						runtime_dir,
						package: package.to_string(),
						chain,
					})
				}
				RuntimeCrateSearchOption::Package(package) => {
					let chain = package.replace("-runtime", "");
					let runtime_dir = PathBuf::from("runtime").join(&chain);
					let cargo_toml = input.workdir.join(&runtime_dir).join("Cargo.toml");
					let _: Value = fs::read_to_string(cargo_toml)?.parse()?;
					Ok(RuntimeCrate {
						workdir: input.workdir.to_owned(),
						runtime_dir,
						package: package.to_string(),
						chain,
					})
				}
				RuntimeCrateSearchOption::ChainName(chain) => {
					let package = format!("{}-runtime", chain);
					let runtime_dir = PathBuf::from("runtime").join(&chain);
					let cargo_toml = input.workdir.join(&runtime_dir).join("Cargo.toml");
					let _: Value = fs::read_to_string(cargo_toml)?.parse()?;

					Ok(RuntimeCrate {
						workdir: input.workdir.to_owned(),
						runtime_dir,
						package,
						chain: chain.to_string(),
					})
				}
			},
			None => todo!("This feature is not implemented yet, please pass one other search criteria"),
		}
	}
}

#[cfg(test)]
mod test_runtime_crate {
	use super::*;

	#[test]
	#[ignore = "local data"]
	#[should_panic] // Not implemented yet, this may end up passing later
	fn test_search_workdir_only() {
		let _ = RuntimeCrate::search(&RuntimeCrateSearchInfo { workdir: "/projects/polkadot".into(), options: None });
	}

	#[test]
	#[ignore = "local data"]
	#[should_panic] // Not implemented yet
	fn test_search_bad_workdir_only() {
		// Should fail for now
		let _ = RuntimeCrate::search(&RuntimeCrateSearchInfo { workdir: "/tmp".into(), options: None });
	}

	#[test]
	#[ignore = "local data"]
	fn test_search_workdir_runtime() {
		// The best way
		let res = RuntimeCrate::search(&RuntimeCrateSearchInfo {
			workdir: "/projects/polkadot".into(),
			options: Some(RuntimeCrateSearchOption::RuntimeDir("runtime/polkadot".into())),
		});
		assert!(res.is_ok());
		println!("res = {:#?}", res);
	}

	#[test]
	#[ignore = "local data"]
	fn test_search_workdir_bad_runtime_dir() {
		// Should fail
		let res = RuntimeCrate::search(&RuntimeCrateSearchInfo {
			workdir: "/projects/polkadot".into(),
			options: Some(RuntimeCrateSearchOption::RuntimeDir("foobar".into())),
		});
		assert!(res.is_err());
		println!("res = {:#?}", res);
	}

	#[test]
	#[ignore = "local data"]
	fn test_search_workdir_package() {
		let res = RuntimeCrate::search(&RuntimeCrateSearchInfo {
			workdir: "/projects/polkadot".into(),
			options: Some(RuntimeCrateSearchOption::Package("polkadot-runtime".into())),
		});
		assert!(res.is_ok());
		println!("res = {:#?}", res);
	}

	#[test]
	#[ignore = "local data"]
	fn test_search_workdir_chain_name() {
		let res = RuntimeCrate::search(&RuntimeCrateSearchInfo {
			workdir: "/projects/polkadot".into(),
			options: Some(RuntimeCrateSearchOption::ChainName("polkadot".into())),
		});
		assert!(res.is_ok());
		println!("res = {:#?}", res);
	}
}
