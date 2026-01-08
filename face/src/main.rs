mod network;

fn main() {
	let token = "ab29c880e47621b6"; // TODO: change me to your token!

	let img_path = crate::network::download_img(token);

	// TODO: do the facial recognition
	// let mut face_tiles = Vec::new();

	// crate::network::send_secret(face_tiles, token);
}
