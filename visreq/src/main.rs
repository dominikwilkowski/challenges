mod calc;
mod network;
mod ocr;

fn main() {
	let token = "xxx"; // TODO: change me to your token!

	let img_path = crate::network::download_img(token);
	if let Ok(lines) = crate::ocr::get_text(&img_path) {
		let result = crate::calc::calc_lines(lines);

		crate::network::send_secret(result, token);
	}
}
