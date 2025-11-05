use hound::{WavReader, WavSamples};

fn main() {
	let mut reader = WavReader::open("touch_tone.wav").unwrap();
	let spec = reader.spec();
	let samples = reader.samples::<i16>().filter_map(Result::ok).collect::<Vec<_>>();
	println!("{spec:?}\n{}", samples.len());
}
