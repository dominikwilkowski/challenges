use std::f32::consts::PI;

const LOWS: [f32; 4] = [697.0, 770.0, 852.0, 941.0];
const HIGHS: [f32; 4] = [1209.0, 1336.0, 1477.0, 1633.0];
const KEYPAD: [[char; 3]; 4] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9'], ['*', '0', '#']];

/// Computes the *single-frequency power* in a short frame using the Goertzel
/// algorithm
fn goertzel_power(frame_samples: &[i16], sample_rate_hz: f32, target_freq_hz: f32) -> f32 {
	let angle_per_sample = 2.0 * PI * target_freq_hz / sample_rate_hz;
	let two_cos_angle = 2.0 * angle_per_sample.cos();

	let mut state_prev = 0.0;
	let mut state_prev2 = 0.0;

	for &raw in frame_samples {
		let sample = raw as f32 / 32768.0;
		let state_curr = sample + two_cos_angle * state_prev - state_prev2;

		state_prev2 = state_prev;
		state_prev = state_curr;
	}

	let real_part = state_prev - state_prev2 * angle_per_sample.cos();
	let imag_part = state_prev2 * angle_per_sample.sin();

	real_part * real_part + imag_part * imag_part
}

/// Convert a single-bin Goertzel/DFT power to dBFS
///
/// Reference = a full-scale sine (amplitude 1.0) that lands exactly on the bin
/// when using a rectangular window; that reference has power ≈ (N/2)^2.
/// If you switch to a Hann window, the coherent gain changes (~-6 dB),
/// so this absolute number shifts; the *relative* differences (which we use to
/// pick tones) remain meaningful either way.
fn power_to_dbfs(power_at_bin: f32, frame_len: usize) -> f32 {
	let full_scale_onbin_power = (frame_len as f32 * 0.5).powi(2);
	10.0 * (power_at_bin / (full_scale_onbin_power + 1e-20)).log10()
}

/// Return the index of the largest element, its value, and the runner-up value
///
/// We keep the runner-up to enforce "separation" (in dB) between the winner and
/// the next strongest bin, which is a standard trick to reduce confusions from
/// nearby bins or leakage.
fn argmax_and_second(values: &[f32; 4]) -> (usize, f32, f32) {
	let mut indices = [0usize, 1, 2, 3];
	indices.sort_by(|&a, &b| values[b].partial_cmp(&values[a]).unwrap_or(std::cmp::Ordering::Equal));
	let best_idx = indices[0];
	(best_idx, values[best_idx], values[indices[1]])
}

/// Slide ~50 ms frames (50% overlap), compute 8 dBFS values per frame via
/// Goertzel, and print the detected DTMF sequence with light debouncing.
/// 50 ms is long enough to resolve DTMF reliably while keeping latency low; 50%
/// overlap reduces the chance that a tone edge straddling frame boundaries is
/// missed.
pub fn detect_digits(samples: &[i16], sample_rate_hz: f32) -> String {
	let frame_len = ((sample_rate_hz * 0.050).round() as usize).max(64);
	let hop_len = (frame_len / 2).max(1);

	const MIN_DBFS: f32 = -40.0;
	const SEP_DB: f32 = 6.0;
	const HOLD_FRAMES: usize = 2;

	let mut active_digit: Option<char> = None;
	let mut active_hold_frames = 0usize;
	let mut digits_out = String::new();

	let mut frame_start = 0usize;
	while frame_start + frame_len <= samples.len() {
		let frame = &samples[frame_start..frame_start + frame_len];

		let mut low_group_dbfs = [0.0; 4];
		let mut high_group_dbfs = [0.0; 4];

		for (k, &freq_hz) in LOWS.iter().enumerate() {
			let p = goertzel_power(frame, sample_rate_hz, freq_hz);
			low_group_dbfs[k] = power_to_dbfs(p, frame_len);
		}
		for (k, &freq_hz) in HIGHS.iter().enumerate() {
			let p = goertzel_power(frame, sample_rate_hz, freq_hz);
			high_group_dbfs[k] = power_to_dbfs(p, frame_len);
		}

		// Pick the dominant low and high bins and ensure each is clearly dominant
		// within its group; this avoids false positives from harmonics/leakage.
		let (low_idx, low_max_db, low_runner_db) = argmax_and_second(&low_group_dbfs);
		let (high_idx, high_max_db, high_runner_db) = argmax_and_second(&high_group_dbfs);

		let mut candidate_digit: Option<char> = None;
		let low_ok = low_max_db > MIN_DBFS && (low_max_db - low_runner_db) >= SEP_DB;
		let high_ok = high_max_db > MIN_DBFS && (high_max_db - high_runner_db) >= SEP_DB;
		if low_ok && high_ok {
			candidate_digit = Some(KEYPAD[low_idx][high_idx]);
		}

		// Debounce: only emit when the same candidate persists across HOLD_FRAMES.
		match (active_digit, candidate_digit) {
			(Some(d), Some(d2)) => {
				if d == d2 {
					active_hold_frames += 1;
				} else {
					if active_hold_frames >= HOLD_FRAMES {
						digits_out.push(d);
					}
					active_digit = Some(d2);
					active_hold_frames = 1;
				}
			},
			(Some(d), None) => {
				if active_hold_frames >= HOLD_FRAMES {
					digits_out.push(d);
				}
				active_digit = None;
				active_hold_frames = 0;
			},
			(None, Some(d2)) => {
				active_digit = Some(d2);
				active_hold_frames = 1;
			},
			(None, None) => { /* nothing stable in this frame */ },
		}

		frame_start += hop_len;
	}

	// Flush any trailing stable digit at end of stream.
	if let Some(d) = active_digit
		&& active_hold_frames >= HOLD_FRAMES
	{
		digits_out.push(d);
	}

	digits_out
}
