const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

pub struct Passwords {
	len: usize,
	max_len: usize,
	indices: Vec<usize>,
}

impl Passwords {
	pub fn new(min_len: usize, max_len: usize) -> Self {
		Self {
			len: min_len,
			max_len,
			indices: vec![0; min_len],
		}
	}
}

impl Iterator for Passwords {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.len > self.max_len {
			return None;
		}

		let password = self.indices.iter().map(|&i| CHARSET[i] as char).collect::<String>();

		let mut pos = self.len;
		loop {
			if pos == 0 {
				// we exhausted this length, move to next length
				self.len += 1;
				if self.len > self.max_len {
					// will return None in the next iteration
					break;
				}

				self.indices = vec![0; self.len];
				break;
			}
			// we go right to left like a type writer (or a numbering system)
			pos -= 1;

			self.indices[pos] += 1;
			if self.indices[pos] < CHARSET.len() {
				break;
			} else {
				self.indices[pos] = 0;
			}
		}

		Some(password)
	}
}
