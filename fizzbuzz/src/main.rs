use std::{
	fmt,
	io::{self, Read},
};

#[derive(Debug)]
enum Term {
	Value(u64),
	Fizz,
	Buzz,
	FizzBuzz,
}

impl fmt::Display for Term {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Term::Value(number) => write!(f, "{number}"),
			Term::Fizz => write!(f, "Fizz"),
			Term::Buzz => write!(f, "Buzz"),
			Term::FizzBuzz => write!(f, "FizzBuzz"),
		}
	}
}

fn fizz_buzz_me(n: u64) -> Term {
	match (n % 3, n % 5) {
		(0, 0) => Term::FizzBuzz,
		(0, _) => Term::Fizz,
		(_, 0) => Term::Buzz,
		_ => Term::Value(n),
	}
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
