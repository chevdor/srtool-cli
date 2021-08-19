use crate::rustc_version::RustcVersion;
use semver::Version;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct SrtoolTag {
	/// This is the srtool version: nightly-2021-03-15 or 1.53.0 for instance.
	pub rustc: RustcVersion,

	/// This is the version of srtool itself. Typicaly something like 0.9.25
	pub srtool: Option<Version>,
}

impl std::fmt::Display for SrtoolTag {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl FromStr for SrtoolTag {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut splitter = s.split('-');
		let rustc = splitter.next().unwrap_or("");
		let srtool = splitter.next().unwrap_or("");

		Ok(Self { rustc: RustcVersion::from_str(rustc).unwrap(), srtool: Version::from_str(srtool).ok() })
	}
}

impl SrtoolTag {
	/// whether the tag is fully qualified (such as 1.53.0-0.9.15) or not (such as 1.53.0)
	pub fn is_fully_qualified(&self) -> bool {
		self.srtool.is_some()
	}
}

#[cfg(test)]
mod test_srtooltah {
	use super::*;

	#[test]
	fn test_from() {
		let tag = SrtoolTag::from_str("1.53.0-0.9.15").unwrap();
		println!("tag = {:?}", tag);
		assert!(tag.is_fully_qualified());
		assert!(tag.rustc == RustcVersion::Stable(Version::from_str("1.53.0").unwrap()));

		let tag = SrtoolTag::from_str("1.53.0").unwrap();
		println!("tag = {:?}", tag);
		assert!(!tag.is_fully_qualified());
	}
}
