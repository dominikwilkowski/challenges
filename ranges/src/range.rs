#[derive(Debug)]
pub enum RangeError {
	ParsingError(String),
}

impl std::fmt::Display for RangeError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			RangeError::ParsingError(error) => write!(f, "Parsing error: {}", error),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Ranges {
	Range { from: u64, to: u64 },
	Scalar(u64),
}

#[derive(Debug)]
pub struct Range {
	numbers: Vec<u64>,
}

impl Range {
	pub fn new(input: &str) -> Result<Self, RangeError> {
		let mut numbers = input
			.split(',')
			.map(|number| {
				number
					.parse::<u64>()
					.map_err(|error| RangeError::ParsingError(format!("Failed to parse '{}': {}", number, error)))
			})
			.collect::<Result<Vec<u64>, _>>()?;
		numbers.sort();

		Ok(Self { numbers })
	}

	pub fn get_range(&self) -> Vec<Ranges> {
		let mut output = Vec::new();
		let mut start_range = None;
		let mut iterations = 0;

		if self.numbers.len() == 1 {
			return vec![Ranges::Scalar(self.numbers[0])];
		}

		for num in &self.numbers {
			if start_range.is_none() {
				start_range = Some(*num);
			} else if let Some(start) = start_range {
				iterations += 1;
				println!("start={start} num={num} iterations={iterations} range={}", *num == start + iterations);
				if start != *num - iterations {
					println!("!!!");
					if start < *num && *num == start + iterations {
						output.push(Ranges::Range { from: start, to: *num });
					} else {
						output.push(Ranges::Scalar(start));
					}
					start_range = Some(*num);
					iterations = 0;
				}
			}
		}

		println!(
			"start={start_range:?} num={} iterations={iterations} range={}",
			self.numbers[self.numbers.len() - 1],
			self.numbers[self.numbers.len() - 1] == start_range.unwrap() + iterations
		);
		if let Some(start) = start_range {
			if start < self.numbers[self.numbers.len() - 1] && self.numbers[self.numbers.len() - 1] == start + iterations {
				output.push(Ranges::Range {
					from: start,
					to: self.numbers[self.numbers.len() - 1],
				});
			} else {
				output.push(Ranges::Scalar(self.numbers[self.numbers.len() - 1]));
			}
		}
		output
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn range_test() {
		let range = Range::new("1,2,3,4,5").unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Range { from: 1, to: 5 }]);

		let range = Range::new("6,3,4,5").unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Range { from: 3, to: 6 }]);
	}

	#[test]
	fn scalar_test() {
		let range = Range::new("1").unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Scalar(1)]);

		let range = Range::new("5,3,1").unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Scalar(1), Ranges::Scalar(3), Ranges::Scalar(5)]);
	}

	#[test]
	fn mixed_test() {
		let range = Range::new("1,3,4,8,10").unwrap();
		assert_eq!(
			range.get_range(),
			vec![
				Ranges::Scalar(1),
				Ranges::Range { from: 3, to: 4 },
				Ranges::Scalar(8),
				Ranges::Scalar(10)
			]
		);

		let range = Range::new("10,3,4,5,1").unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Scalar(1), Ranges::Range { from: 3, to: 5 }, Ranges::Scalar(10)]);
	}
}
