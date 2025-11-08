use hound::WavReader;

mod network;
mod tone;

fn main() {
	let token = "xxx"; // TODO: change me to your token!

	crate::network::download_wav(token);

	let mut reader = WavReader::open("temp/tone.wav").unwrap();
	let spec = reader.spec();
	let samples = reader.samples().filter_map(Result::ok).collect::<Vec<i16>>();
	let tones = crate::tone::detect_digits(&samples, spec.sample_rate as f32);
	crate::network::send_secret(tones, token);
}
