use oar_ocr::prelude::*;

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let ocr = OAROCRBuilder::new(
		"models/ppocrv4_server_det.onnx".to_string(),
		"models/en_ppocrv4_mobile_rec.onnx".to_string(),
		"models/en_dict.txt".to_string(),
	)
	.build()?;

	let image = oar_ocr::utils::load_image(Path::new("temp/img.png"))?;
	let results = ocr.predict(&[image])?;
	let result = &results[0];

	// Print extracted text with confidence scores using the modern TextRegion API
	for text_region in &result.text_regions {
		if let (Some(text), Some(confidence)) = (&text_region.text, text_region.confidence) {
			println!("Text: {} (confidence: {:.2})", text, confidence);
		}
	}

	Ok(())
}
