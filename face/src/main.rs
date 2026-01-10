mod img;
mod network;

fn main() {
	let token = "ab29c880e47621b6"; // TODO: change me to your token!

	let img_path = crate::network::download_img(token);

	let mut detector = rustface::create_detector("model/seeta_fd_frontal_v1.0.bin").unwrap();
	detector.set_score_thresh(0.95);

	let gray = image::open(img_path).unwrap().to_luma8();
	let width = gray.width() as usize;
	let height = gray.height() as usize;

	let cell_width = width / 8;
	let cell_height = height / 8;
	let mut face_tiles: Vec<[usize; 2]> = Vec::new();

	for r in 0..8 {
		for c in 0..8 {
			let column = c * cell_width;
			let row = r * cell_height;
			let x1 = if c == 7 { width } else { (c + 1) * cell_width };
			let y1 = if r == 7 { height } else { (r + 1) * cell_height };

			let w = x1 - column;
			let h = y1 - row;

			let cell = img::get_cell(&gray, column, row, w, h);
			if img::cell_has_face(&mut detector, cell, w, h) {
				face_tiles.push([row / 100, column / 100]);
			}
		}
	}

	println!("{face_tiles:?}");

	crate::network::send_secret(face_tiles, token);
}
