#![cfg(test)]
const CLI: &str = "srtool";

#[cfg(test)]
mod cli_tests {

	#[cfg(test)]
	mod help {
		use crate::CLI;
		use assert_cmd::Command;

		#[test]
		fn it_shows_help() {
			let mut cmd = Command::cargo_bin(CLI).unwrap();
			let assert = cmd.arg("--help").assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod version {
		use crate::CLI;
		use assert_cmd::Command;

		#[test]
		fn it_lints_good_file() {
			let mut cmd = Command::cargo_bin(CLI).unwrap();
			let assert = cmd.arg("version").assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod pull {
		use crate::CLI;
		use assert_cmd::Command;

		#[test]
		fn it_calls_list() {
			let mut cmd = Command::cargo_bin(CLI).unwrap();
			let assert = cmd.arg("pull").assert();
			assert.success().code(0);
		}
	}
}
