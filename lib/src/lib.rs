use log::{debug, info};
use std::{
	fs::{self, File},
	io::Write,
	time::{Duration, SystemTime},
};

const CACHE_FILE: &str = "srtool-tag-latest.txt";

/// Fetch the latest image tag
pub fn fetch_image_tag() -> Result<String, ureq::Error> {
	debug!("Fetching latest version from gitlab");
	let url = "https://gitlab.com/chevdor/srtool/-/raw/master/RUSTC_VERSION";
	let tag: String = ureq::get(url).set("Content-Type", "application/txt").call()?.into_string()?;
	debug!("tag: {}", tag);
	Ok(tag)
}

/// Get the latest image. it is fetched from cache we have a version that is younger than `cache_validity` in seconds.
pub fn get_image_tag(cache_validity: Option<u64>) -> Result<String, ureq::Error> {
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
				debug!("cache is {:.2?} seconds old", elapsed);
			}
		}
	} else {
		use_cache = false;
		debug!("no cache");
	}

	let cached_value = fs::read_to_string(&cache_location).map(|s| s.trim_end().to_string());

	match cached_value {
		Ok(value) if use_cache => {
			info!("using cached value: {:?}", value);
			Ok(value)
		}
		_ => {
			let value = fetch_image_tag()?;
			let mut file = File::create(cache_location)?;
			file.write_all(value.as_bytes())?;

			Ok(value)
		}
	}

	// if cached_value.is_ok() && use_cache {
	// 	info!("using cached value: {:?}", cached_value);

	// 	return Ok(cached_value.unwrap());
	// } else {
	// 	let value = fetch_image_tag()?;
	// 	let mut file = File::create(cache_location)?;
	// 	file.write_all(value.as_bytes())?;
	// }

	// Ok(String::from("nightly-2021-03-15"))
}

pub fn clear_cache() {
	let cache_location = std::env::temp_dir().join(CACHE_FILE);
	debug!("Deleting cache from {}", cache_location.display());
	let _ = fs::remove_file(cache_location);
}

#[cfg(test)]
mod tests {
	use crate::fetch_image_tag;

	#[test]
	fn it_fetches_the_version() {
		let tag = fetch_image_tag().unwrap();
		println!("current tag = {:?}", tag);
		assert!(tag.len() > 0);
	}
}
