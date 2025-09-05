#[derive(Debug, PartialEq)]
pub enum RangeError {
	ParsingError(String),
}

impl std::fmt::Display for RangeError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			RangeError::ParsingError(error) => write!(f, "{error}"),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Ranges {
	Range { from: u64, to: u64 },
	Scalar(u64),
}

impl std::fmt::Display for Ranges {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Ranges::Range { from, to } => write!(f, "{from}-{to}"),
			Ranges::Scalar(value) => write!(f, "{value}"),
		}
	}
}

#[derive(Debug, PartialEq)]
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
		let mut result = Vec::new();

		for num in &self.numbers {
			if result.is_empty() {
				// very first iteration
				result.push(Ranges::Scalar(*num));
			} else {
				// every other iteration
				match result.pop().unwrap() {
					Ranges::Scalar(last_num) => {
						// are we within a range?
						if last_num + 1 == *num {
							result.push(Ranges::Range {
								from: last_num,
								to: *num,
							});
						} else {
							result.push(Ranges::Scalar(last_num));
							result.push(Ranges::Scalar(*num));
						}
					},
					Ranges::Range { from, to } => {
						// are we within a range?
						if to + 1 == *num {
							result.push(Ranges::Range { from, to: *num });
						} else {
							result.push(Ranges::Range { from, to });
							result.push(Ranges::Scalar(*num));
						}
					},
				};
			}
		}

		result
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

		let range = Range::new("1,3,10,20").unwrap();
		assert_eq!(
			range.get_range(),
			vec![
				Ranges::Scalar(1),
				Ranges::Scalar(3),
				Ranges::Scalar(10),
				Ranges::Scalar(20),
			]
		);

		let range = Range::new("5,3,1").unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Scalar(1), Ranges::Scalar(3), Ranges::Scalar(5)]);
	}

	#[test]
	fn mixed_test() {
		let range = Range::new("1,3,4").unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Scalar(1), Ranges::Range { from: 3, to: 4 },]);

		let range = Range::new("1,3,4,8,10").unwrap();
		assert_eq!(
			range.get_range(),
			vec![
				Ranges::Scalar(1),
				Ranges::Range { from: 3, to: 4 },
				Ranges::Scalar(8),
				Ranges::Scalar(10),
			]
		);

		let range = Range::new("1,2,3,8,10,11").unwrap();
		assert_eq!(
			range.get_range(),
			vec![
				Ranges::Range { from: 1, to: 3 },
				Ranges::Scalar(8),
				Ranges::Range { from: 10, to: 11 },
			]
		);

		let range = Range::new("1,2,4,5,6,9,10,12").unwrap();
		assert_eq!(
			range.get_range(),
			vec![
				Ranges::Range { from: 1, to: 2 },
				Ranges::Range { from: 4, to: 6 },
				Ranges::Range { from: 9, to: 10 },
				Ranges::Scalar(12),
			]
		);

		let range = Range::new("10,3,4,5,1").unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Scalar(1), Ranges::Range { from: 3, to: 5 }, Ranges::Scalar(10)]);
	}

	#[test]
	fn parsing_error_test() {
		assert_eq!(
			Range::new(""),
			Err(RangeError::ParsingError(String::from("Failed to parse '': cannot parse integer from empty string")))
		);

		assert_eq!(
			RangeError::ParsingError(String::from("Failed to parse '': cannot parse integer from empty string")).to_string(),
			String::from("Failed to parse '': cannot parse integer from empty string")
		);

		assert_eq!(
			Range::new("1,2,three"),
			Err(RangeError::ParsingError(String::from("Failed to parse 'three': invalid digit found in string")))
		);

		assert_eq!(
			Range::new("1,2,-3"),
			Err(RangeError::ParsingError(String::from("Failed to parse '-3': invalid digit found in string")))
		);

		assert_eq!(
			Range::new("1,2,3.5"),
			Err(RangeError::ParsingError(String::from("Failed to parse '3.5': invalid digit found in string")))
		);

		assert_eq!(
			Range::new("1,2,18446744073709551616"),
			Err(RangeError::ParsingError(String::from(
				"Failed to parse '18446744073709551616': number too large to fit in target type"
			)))
		);
	}
}
