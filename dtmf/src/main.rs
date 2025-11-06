use hound::WavReader;

mod tone;

fn main() {
	let mut reader = WavReader::open("temp/touch_tone.wav").unwrap();
	let spec = reader.spec();
	let samples = reader.samples().filter_map(Result::ok).collect::<Vec<i16>>();
	let tones = crate::tone::detect_digits(&samples, spec.sample_rate as f32);
	println!("{tones}");
}
