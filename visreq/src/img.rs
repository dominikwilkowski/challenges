use image::{imageops, open};

pub fn gray_and_contrast(in_path: &str, out_path: &str) -> String {
	let img = open(in_path).unwrap();
	let gray = imageops::grayscale(&img);
	let darken = imageops::brighten(&gray, -80);
	let boosted = imageops::contrast(&darken, 80.0);

	boosted.save(out_path).unwrap();
	out_path.to_string()
}
