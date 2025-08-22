#[derive(Debug)]
pub enum RangeError {
	InvalidInput(String),
	ParsingError(String),
}

impl std::fmt::Display for RangeError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			RangeError::InvalidInput(input) => write!(f, "Invalid input: {}", input),
			RangeError::ParsingError(error) => write!(f, "Parsing error: {}", error),
		}
	}
}

#[derive(Debug)]
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
		let numbers = input
			.split(',')
			.map(|number| {
				number
					.parse::<u64>()
					.map_err(|error| RangeError::ParsingError(format!("Failed to parse '{}': {}", number, error)))
			})
			.collect::<Result<Vec<u64>, _>>()?;

		Ok(Self { numbers })
	}

	pub fn get_range(&self) -> Vec<Ranges> {
		vec![]
	}
}
