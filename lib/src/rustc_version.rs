use semver::Version;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RustcVersion {
	Stable(Version),
	// Beta(String),
	Nightly(String),
	// Dev(String),
}

// pub struct Error;

impl FromStr for RustcVersion {
	type Err = String;

	// TODO: The checks below are very light ... See https://docs.rs/rustc_version/0.2.3/rustc_version/
	fn from_str(s: &str) -> Result<RustcVersion, Self::Err> {
		println!("s = {:?}", s);
		match semver::Version::from_str(s) {
			Ok(version) => Ok(RustcVersion::Stable(version)),
			_ => Ok(RustcVersion::Nightly(s.to_string())),
		}
	}
}

#[cfg(test)]
mod test_rustc_version {
	use super::*;

	#[test]
	fn test_from_str() {
		println!("v = {:?}", RustcVersion::from_str("1.53.0"));
		println!("v = {:?}", RustcVersion::from_str("nightly-2021-03-15"));
		println!("v = {:?}", RustcVersion::from_str("rustc 1.53.0 (53cb7b09b 2021-06-17)"));

		// TODO: the following should error
		// println!("v = {:?}", RustcVersion::from_str("junk"));
	}
}
