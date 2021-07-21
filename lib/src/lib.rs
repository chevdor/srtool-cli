mod digest;

mod run_specs;
mod runner;
mod runtime_crate;
mod rustc_version;
mod samples;
mod srtool_tag;
mod version;

pub use digest::*;
pub use run_specs::*;
pub use runner::*;
pub use runtime_crate::*;
pub use srtool_tag::*;

use log::{debug, info};
use std::{
	env,
	fs::{self, File},
	io::Write,
	process::Command,
	time::{Duration, SystemTime},
};

const CACHE_FILE: &str = "srtool-tag-latest.txt";

/// Fetch the latest image tag
pub fn fetch_image_tag() -> Result<String, ureq::Error> {
	debug!("Fetching latest version from github");
	let url = "https://raw.githubusercontent.com/paritytech/srtool/master/RUSTC_VERSION";
	let tag: String = ureq::get(url).set("Content-Type", "application/txt").call()?.into_string()?.trim().to_string();
	debug!("tag: {}", tag);
	Ok(tag)
}

/// Get the latest image. it is fetched from cache we have a version that is younger than `cache_validity` in seconds.
pub fn get_image_tag(cache_validity: Option<u64>) -> Result<String, ureq::Error> {
	let env_tag = env::var("SRTOOL_TAG");
	if let Ok(tag) = env_tag {
		info!("using tag from ENV: {:?}", tag);
		return Ok(tag);
	}

	let cache_location = std::env::temp_dir().join(CACHE_FILE);
	debug!("cache_location = {:?}", cache_location);

	let mut use_cache: bool = false;
	if cache_location.exists() {
		let metadata = fs::metadata(&cache_location).unwrap();
		let last_modif = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
		let validity = cache_validity.map(Duration::from_secs);

		if let Some(duration) = validity {
			let elapsed = last_modif.elapsed().unwrap();
			if elapsed < duration {
				use_cache = true;
				debug!("cache is {:.2?} old (< max {:.2?})", elapsed, duration);
			}
		}
	} else {
		use_cache = false;
		debug!("no cache");
	}

	let cached_value = fs::read_to_string(&cache_location).map(|s| s.trim_end().to_string());

	match cached_value {
		Ok(value) if use_cache => {
			info!("using tag from cached value: {:?}", value);
			Ok(value)
		}
		_ => {
			let value = fetch_image_tag()?;
			let mut file = File::create(cache_location)?;
			file.write_all(value.as_bytes())?;
			info!("using tag from fetched value: {:?}", value);

			Ok(value)
		}
	}
}

pub fn clear_cache() {
	let cache_location = std::env::temp_dir().join(CACHE_FILE);
	debug!("Deleting cache from {}", cache_location.display());
	let _ = fs::remove_file(cache_location);
}

// docker inspect paritytech/srtool:1.53.0 | jq -r '.[0].RepoDigests[0]'
pub fn get_image_digest(image: &str, tag: &str) -> Option<String> {
	let command = format!("docker inspect {image}:{tag}", image = image, tag = tag);

	let output = if cfg!(target_os = "windows") {
		Command::new("cmd").args(&["/C", command.as_str()]).output()
	} else {
		Command::new("sh").arg("-c").arg(command).output()
	};

	let output_str = String::from_utf8(output.unwrap().stdout).unwrap_or_else(|_| "".into());
	let json: serde_json::Value = serde_json::from_str(&output_str).unwrap_or_default();
	let digest_str = json[0]["RepoDigests"][0].as_str().unwrap_or_default();
	let digest = digest_str.split('@').nth(1);
	digest.map(String::from)
}

#[cfg(test)]
mod tests {
	use crate::{fetch_image_tag, get_image_digest};

	#[test]
	fn it_fetches_the_version() {
		let tag = fetch_image_tag().unwrap();
		println!("current tag = {:?}", tag);
		assert!(tag.len() > 0);
	}

	#[test]
	#[ignore = "require docker installed + pulling the image"]
	fn it_gets_the_image_digest() {
		let image = "paritytech/srtool";
		let tag = fetch_image_tag().unwrap();
		println!("Checking digest for {}:{}", image, tag);
		let result = get_image_digest(image, &tag);
		assert!(result.is_some());
	}
}
