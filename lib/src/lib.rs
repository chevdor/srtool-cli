/// Fetch the latest image tag
pub fn fetch_image_tag() -> Result<String, ureq::Error> {
	let url = "https://gitlab.com/chevdor/srtool/-/raw/master/RUSTC_VERSION";
	let tag: String = ureq::get(url).set("Content-Type", "application/txt").call()?.into_string()?;
	Ok(tag)
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
