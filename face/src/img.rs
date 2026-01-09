use image::GrayImage;
use rustface::{Detector, ImageData};

pub fn get_cell(gray: &GrayImage, x: usize, y: usize, w: usize, h: usize) -> Vec<u8> {
	let full_w = gray.width() as usize;
	let src = gray.as_raw(); // &[u8], 1 byte per pixel

	let mut out = Vec::with_capacity(w * h);
	let w_us = w;

	for row in 0..h {
		let yy = (y) + row;
		let start = yy * full_w + (x);
		out.extend_from_slice(&src[start..start + w_us]);
	}

	out
}

pub fn cell_has_face(detector: &mut Box<dyn Detector>, tile: Vec<u8>, w: usize, h: usize) -> bool {
	let img = ImageData::new(&tile, w as u32, h as u32);
	!detector.detect(&img).is_empty()
}
