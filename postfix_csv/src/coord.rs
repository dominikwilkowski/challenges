use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coord {
	pub column: usize,
	pub row: usize,
}

impl Coord {
	pub fn parse(input: &str) -> Option<Coord> {
		let mut has_letters = false;
		let mut has_numbers = false;

		let mut column: usize = 0;
		let mut row: usize = 0;

		for item in input.chars() {
			if item.is_ascii_alphabetic() {
				if has_numbers {
					return None;
				}
				let case_conversion = if item.is_ascii_lowercase() {
					'a' as usize - 'A' as usize
				} else {
					0
				};
				column = (26 * column) + (item as usize) - case_conversion - ('A' as usize) + 1;
				has_letters = true;
			} else if item.is_ascii_digit() {
				if !has_letters {
					return None;
				}
				row = row * 10 + (item as usize - '0' as usize);
				has_numbers = true;
			} else {
				return None;
			}
		}

		if has_letters && has_numbers && row > 0 {
			Some(Coord {
				column: column - 1,
				row: row - 1,
			})
		} else {
			None
		}
	}

	pub fn stringify(&self) -> String {
		let mut result = Vec::new();
		let mut column = self.column + 1;

		while column > 0 {
			column -= 1;
			let remainder = (column % 26) as u8;
			let letter = (b'A' + remainder) as char;
			result.push(letter);
			column /= 26;
		}

		result.reverse();

		result.iter().collect::<String>() + &(self.row + 1).to_string()
	}
}

#[test]
fn parse_test() {
	assert_eq!(Coord::parse("A1"), Some(Coord { column: 0, row: 0 }));
	assert_eq!(Coord::parse("a1"), Some(Coord { column: 0, row: 0 }));
	assert_eq!(Coord::parse("a10"), Some(Coord { column: 0, row: 9 }));
	assert_eq!(Coord::parse("a510"), Some(Coord { column: 0, row: 509 }));
	assert_eq!(Coord::parse("B2"), Some(Coord { column: 1, row: 1 }));
	assert_eq!(Coord::parse("Z2"), Some(Coord { column: 25, row: 1 }));
	assert_eq!(Coord::parse("AA2"), Some(Coord { column: 26, row: 1 }));
	assert_eq!(Coord::parse("AB2"), Some(Coord { column: 27, row: 1 }));
	assert_eq!(Coord::parse("AZ2"), Some(Coord { column: 51, row: 1 }));
	assert_eq!(Coord::parse("BA2"), Some(Coord { column: 52, row: 1 }));
	assert_eq!(Coord::parse("BZ2"), Some(Coord { column: 77, row: 1 }));
	assert_eq!(Coord::parse("CA2"), Some(Coord { column: 78, row: 1 }));
	assert_eq!(Coord::parse("ZA2"), Some(Coord { column: 676, row: 1 }));
	assert_eq!(Coord::parse("ZZ2"), Some(Coord { column: 701, row: 1 }));
	assert_eq!(Coord::parse("AAA2"), Some(Coord { column: 702, row: 1 }));
	assert_eq!(Coord::parse("AAB2"), Some(Coord { column: 703, row: 1 }));
	assert_eq!(
		Coord::parse("ZZZ50"),
		Some(Coord {
			column: 18_277,
			row: 49
		})
	);
	assert_eq!(
		Coord::parse("FZZ123412021312"),
		Some(Coord {
			column: 4_757,
			row: 123_412_021_311
		})
	);

	assert_eq!(Coord::parse("A1"), Some(Coord { column: 0, row: 0 }));
	assert_eq!(Coord::parse("AA11"), Some(Coord { column: 26, row: 10 }));
	assert_eq!(Coord::parse("ZZZZ1"), Some(Coord { column: 475253, row: 0 }));
	assert_eq!(Coord::parse("Z1234"), Some(Coord { column: 25, row: 1233 }));
	assert_eq!(Coord::parse("a1"), Some(Coord { column: 0, row: 0 }));

	assert_eq!(Coord::parse("1"), None);
	assert_eq!(Coord::parse("A"), None);
	assert_eq!(Coord::parse("5A"), None);
	assert_eq!(Coord::parse("A1A"), None);
	assert_eq!(Coord::parse("1A1"), None);
	assert_eq!(Coord::parse("A1A1"), None);
	assert_eq!(Coord::parse("AAA123.123"), None);
	assert_eq!(Coord::parse("A 1"), None);
	assert_eq!(Coord::parse("AB2/3"), None);
	assert_eq!(Coord::parse("a-1"), None);
	assert_eq!(Coord::parse("b 5#"), None);
}

#[test]
fn stringify_test() {
	assert_eq!(Coord { column: 0, row: 0 }.stringify(), String::from("A1"));
	assert_eq!(Coord { column: 1, row: 1 }.stringify(), String::from("B2"));
	assert_eq!(Coord { column: 25, row: 1 }.stringify(), String::from("Z2"));
	assert_eq!(Coord { column: 26, row: 4 }.stringify(), String::from("AA5"));
	assert_eq!(Coord { column: 701, row: 4 }.stringify(), String::from("ZZ5"));
	assert_eq!(Coord { column: 702, row: 4 }.stringify(), String::from("AAA5"));
	assert_eq!(
		Coord {
			column: 20_000,
			row: 1000
		}
		.stringify(),
		String::from("ACOG1001")
	);
}

impl fmt::Display for Coord {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.stringify())
	}
}

#[test]
fn to_string_test() {
	assert_eq!(format!("{}", Coord { column: 0, row: 0 }), String::from("A1"));
	assert_eq!(format!("{}", Coord { column: 1, row: 1 }), String::from("B2"));
	assert_eq!(format!("{}", Coord { column: 25, row: 1 }), String::from("Z2"));
	assert_eq!(format!("{}", Coord { column: 26, row: 4 }), String::from("AA5"));
	assert_eq!(format!("{}", Coord { column: 701, row: 4 }), String::from("ZZ5"));
	assert_eq!(format!("{}", Coord { column: 702, row: 4 }), String::from("AAA5"));
	assert_eq!(
		format!(
			"{}",
			Coord {
				column: 20_000,
				row: 1000
			}
		),
		String::from("ACOG1001")
	);
}
