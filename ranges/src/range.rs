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
	pub fn new(input: &String) -> Result<Self, RangeError> {
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

		let mut iter = self.numbers.iter().peekable();
		while let Some(num) = iter.next() {
			if start_range.is_none() {
				start_range = Some(*num);
			} else if let Some(start) = start_range {
				if let Some(next) = iter.peek() {
					println!("num={num} next={next}");
					if *num < *next - 1 {
						if start < *num {
							output.push(Ranges::Range { from: start, to: *num });
							start_range = Some(**next);
						} else {
							output.push(Ranges::Scalar(start));
						}
					}
				}
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
		let range = Range::new(&String::from("1,2,3,4,5")).unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Range { from: 1, to: 5 }]);

		let range = Range::new(&String::from("6,3,4,5")).unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Range { from: 3, to: 6 }]);
	}

	#[test]
	fn scalar_test() {
		let range = Range::new(&String::from("1")).unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Scalar(1)]);

		let range = Range::new(&String::from("5,3,1")).unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Scalar(1), Ranges::Scalar(3), Ranges::Scalar(5)]);
	}

	#[test]
	fn mixed_test() {
		let range = Range::new(&String::from("1,3,4,8,10")).unwrap();
		assert_eq!(
			range.get_range(),
			vec![
				Ranges::Scalar(1),
				Ranges::Range { from: 3, to: 4 },
				Ranges::Scalar(8),
				Ranges::Scalar(10)
			]
		);

		let range = Range::new(&String::from("10,3,4,5,1")).unwrap();
		assert_eq!(range.get_range(), vec![Ranges::Scalar(1), Ranges::Range { from: 3, to: 5 }, Ranges::Scalar(10)]);
	}
}
