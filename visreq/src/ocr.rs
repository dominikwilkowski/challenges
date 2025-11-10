use oar_ocr::prelude::*;

use std::path::Path;

pub fn get_text(img_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
	let ocr = OAROCRBuilder::new(
		"models/ppocrv4_server_det.onnx".to_string(),
		"models/en_ppocrv4_mobile_rec.onnx".to_string(),
		"models/en_dict.txt".to_string(),
	)
	.build()?;

	let image = oar_ocr::utils::load_image(Path::new(img_path))?;
	let results = ocr.predict(&[image])?;

	let numbers =
		results[0].text_regions.iter().filter_map(|region| region.text.as_deref().map(ToOwned::to_owned)).collect();
	Ok(numbers)
}
