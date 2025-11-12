mod calc;
mod img;
mod network;
mod ocr;

fn main() {
	let token = "xxx"; // TODO: change me to your token!

	let img_path = crate::network::download_img(token);
	let img_path = crate::img::gray_and_contrast(&img_path, "temp/img_bw.png");
	let res = crate::ocr::get_text(&img_path);
	if let Ok(lines) = res {
		let result = crate::calc::calc_lines(lines);
		println!("result={result}");

		crate::network::send_secret(result, token);
	} else {
		panic!("Failed to get text from image with {res:#?}");
	}
}
