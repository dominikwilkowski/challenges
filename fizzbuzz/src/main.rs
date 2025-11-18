use std::{
	fmt,
	io::{self, Read},
};

#[derive(Debug)]
enum FizzBuzz {
	Value(u64),
	Fizz,
	Buzz,
	FizzBuzz,
}

impl fmt::Display for FizzBuzz {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			FizzBuzz::Value(number) => write!(f, "{number}"),
			FizzBuzz::Fizz => write!(f, "Fizz"),
			FizzBuzz::Buzz => write!(f, "Buzz"),
			FizzBuzz::FizzBuzz => write!(f, "FizzBuzz"),
		}
	}
}

fn fizz_buzz_me(n: u64) -> FizzBuzz {
	return match (n % 3, n % 5) {
		(0, 0) => FizzBuzz::FizzBuzz,
		(0, _) => FizzBuzz::Fizz,
		(_, 0) => FizzBuzz::Buzz,
		_ => FizzBuzz::Value(n),
	};
}

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let range = input.split_whitespace().map(|s| s.parse::<u64>()).collect::<Result<Vec<u64>, _>>().unwrap();

	for n in range[0]..=range[1] {
		let what = fizz_buzz_me(n);
		println!("{what}");
	}
}
