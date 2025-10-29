use std::fmt::{self, Write};

use crate::{coord::Coord, sheet::Sheet};

#[derive(Debug, PartialEq)]
pub enum PostfixError {
	#[cfg(debug_assertions)]
	RecursionDepthExceeded(Vec<Coord>),
	#[cfg(not(debug_assertions))]
	RecursionDepthExceeded,
	NotEnoughOperands,
	TooManyOperands,
	CellNotFound,
	DivisionByZero,
}

impl fmt::Display for PostfixError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("#ERR")
	}
}

#[test]
fn error_enum_test() {
	#[cfg(debug_assertions)]
	assert_eq!(format!("{}", PostfixError::RecursionDepthExceeded(Vec::new())), String::from("#ERR"));
	#[cfg(not(debug_assertions))]
	assert_eq!(format!("{}", PostfixError::RecursionDepthExceeded), String::from("#ERR"));

	assert_eq!(format!("{}", PostfixError::NotEnoughOperands), String::from("#ERR"));
	assert_eq!(format!("{}", PostfixError::TooManyOperands), String::from("#ERR"));
	assert_eq!(format!("{}", PostfixError::CellNotFound), String::from("#ERR"));
	assert_eq!(format!("{}", PostfixError::DivisionByZero), String::from("#ERR"));
}

#[derive(Debug, PartialEq)]
pub struct Postfix<'a> {
	pub sheet: &'a mut Sheet<'a>,
	stack: Vec<f64>,
}

impl<'a> Postfix<'a> {
	pub fn new(sheet: &'a mut Sheet<'a>) -> Self {
		Self {
			sheet,
			stack: Vec::new(),
		}
	}

	pub fn calc_cell(&self, this_coord: &Coord, cell: &str, call_stack: &mut Vec<Coord>) -> Result<f64, PostfixError> {
		let mut stack = Vec::with_capacity(20);
		let is_recursive_call = !call_stack.is_empty();

		if call_stack.contains(this_coord) {
			#[cfg(debug_assertions)]
			return Err(PostfixError::RecursionDepthExceeded(call_stack.clone()));
			#[cfg(not(debug_assertions))]
			return Err(PostfixError::RecursionDepthExceeded);
		}

		for item in cell.split_ascii_whitespace() {
			match item {
				"+" => {
					let (a, b) = (stack.pop(), stack.pop());
					if let (Some(a), Some(b)) = (a, b) {
						stack.push(b + a);
					} else {
						return Err(PostfixError::NotEnoughOperands);
					}
				},
				"-" => {
					let (a, b) = (stack.pop(), stack.pop());
					if let (Some(a), Some(b)) = (a, b) {
						stack.push(b - a);
					} else {
						return Err(PostfixError::NotEnoughOperands);
					}
				},
				"*" => {
					let (a, b) = (stack.pop(), stack.pop());
					if let (Some(a), Some(b)) = (a, b) {
						stack.push(b * a);
					} else {
						return Err(PostfixError::NotEnoughOperands);
					}
				},
				"/" => {
					let (a, b) = (stack.pop(), stack.pop());
					if let (Some(a), Some(b)) = (a, b) {
						if a == 0.0 {
							return Err(PostfixError::DivisionByZero);
						}
						stack.push(b / a);
					} else {
						return Err(PostfixError::NotEnoughOperands);
					}
				},
				_ => {
					if let Some(coord) = Coord::parse(item) {
						call_stack.push(*this_coord);
						match self.sheet.get(&coord) {
							Some(contents) => {
								match self.calc_cell(&coord, contents, call_stack) {
									Ok(calc_cell) => stack.push(calc_cell),
									Err(error) => return Err(error),
								};
							},
							None => return Err(PostfixError::CellNotFound),
						}
					} else if let Ok(operand) = item.parse::<f64>() {
						stack.push(operand);
					}
				},
			}
		}

		if is_recursive_call && stack.len() == 1 {
			// we are inside a recursive call
			Ok(*stack.last().unwrap())
		} else if stack.len() > 1 {
			Err(PostfixError::TooManyOperands)
		} else if stack.is_empty() {
			Err(PostfixError::NotEnoughOperands)
		} else {
			Ok(stack[0])
		}
	}

	pub fn process_sheet(&self, capacity: usize) -> String {
		let mut output = String::with_capacity(capacity);
		let mut call_stack = Vec::with_capacity(100);

		for row in 0..self.sheet.data.len() {
			if row != 0 {
				output.push('\n');
			}
			for column in 0..self.sheet.data[row].len() {
				let cell = self.sheet.data[row][column];

				if column != 0 {
					output.push(self.sheet.separator);
				}

				call_stack.clear();
				match self.calc_cell(&Coord { column, row }, cell, &mut call_stack) {
					Ok(result) => write!(&mut output, "{}", result).unwrap(),
					Err(error) => write!(&mut output, "{}", error).unwrap(),
				};
			}
		}
		output
	}
}

#[test]
fn new_test() {
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["x x"]],
			separator: ',',
		}),
		Postfix {
			sheet: &mut Sheet {
				data: vec![vec!["x x"]],
				separator: ',',
			},
			stack: Vec::new(),
		}
	);
}

#[test]
fn calc_cell_test() {
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ','
		})
		.calc_cell(&Coord { column: 0, row: 0 }, " 2    3 + ", &mut Vec::new()),
		Ok(5.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "2 3 +", &mut Vec::new()),
		Ok(5.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "2 5 3 * -", &mut Vec::new()),
		Ok(-13.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "6 2 /", &mut Vec::new()),
		Ok(3.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "          8 ? 19.5 # 6  / : * 3  1.5  14 - +      * ", &mut Vec::new()),
		Ok(-247.0)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "6 /", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "62/", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "6 2 / +", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "6 /", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: Vec::new(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "?", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 2 +"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "A1 2 +", &mut Vec::new()),
		Ok(7.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 C1 +", "2"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "A1 C1 +", &mut Vec::new()),
		Ok(7.0)
	);

	let data = vec![
		vec!["B1 B2 +", "2 B2 3 * -", "+", "26"],
		vec!["A1", "5", "7 2 /", "2 20 * 2 / 3 4 + 3 2 * * + 6 - 15 +"],
		vec!["C2 3 *", "1 B4", "5 1 2 + 4 * + 3 -", "0.08 6 15 *"],
		vec!["5 7 7 - /", "67.5 B3 *", "-14 A5 +", ""],
	];
	// ROW 1
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 0 }, "B1 B2 +", &mut Vec::new()),
		Ok(-8.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "2 B2 3 * -", &mut Vec::new()),
		Ok(-13.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 2, row: 0 }, "+", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 3, row: 0 }, "26", &mut Vec::new()),
		Ok(26.0)
	);

	// ROW 2
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 1 }, "A1", &mut Vec::new()),
		Ok(-8.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 1 }, "5", &mut Vec::new()),
		Ok(5.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 2, row: 1 }, "7 2 /", &mut Vec::new()),
		Ok(3.5)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 3, row: 1 }, "2 20 * 2 / 3 4 + 3 2 * * + 6 - 15 +", &mut Vec::new()),
		Ok(71.0)
	);

	// ROW 3
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 2 }, "C2 3 *", &mut Vec::new()),
		Ok(10.5)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 2 }, "1 B4", &mut Vec::new()),
		Err(PostfixError::RecursionDepthExceeded(vec![Coord { column: 1, row: 2 }, Coord { column: 1, row: 3 }]))
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 2, row: 2 }, "5 1 2 + 4 * + 3 -", &mut Vec::new()),
		Ok(14.0)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 3, row: 2 }, "0.08 6 15 *", &mut Vec::new()),
		Err(PostfixError::TooManyOperands)
	);

	// ROW 4
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 0, row: 3 }, "5 7 7 - /", &mut Vec::new()),
		Err(PostfixError::DivisionByZero)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 3 }, "67.5 B3 *", &mut Vec::new()),
		Err(PostfixError::RecursionDepthExceeded(vec![Coord { column: 1, row: 3 }, Coord { column: 1, row: 2 }]))
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data.clone(),
			separator: ',',
		})
		.calc_cell(&Coord { column: 2, row: 3 }, "-14 A5 +", &mut Vec::new()),
		Err(PostfixError::CellNotFound)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: data,
			separator: ',',
		})
		.calc_cell(&Coord { column: 3, row: 3 }, "", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);

	// Edge cases
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["B1", "A1 C1 +", "2"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "A1 C1 +", &mut Vec::new()),
		Err(PostfixError::RecursionDepthExceeded(vec![Coord { column: 1, row: 0 }, Coord { column: 0, row: 0 }]))
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5 5", "5 C1 +", "A1"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "5 C1 +", &mut Vec::new()),
		Err(PostfixError::TooManyOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "D1 C1 +", "A1"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "D1 C1 +", &mut Vec::new()),
		Err(PostfixError::CellNotFound)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A2 C1 +", "A1"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "A2 C1 +", &mut Vec::new()),
		Err(PostfixError::CellNotFound)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5 -", "4 C1 +", "A1"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "4 C1 +", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["0", "4 C1 /", "A1"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "4 C1 /", &mut Vec::new()),
		Err(PostfixError::DivisionByZero)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 C1 + +", "A1 1"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "A1 C1 + +", &mut Vec::new()),
		Err(PostfixError::TooManyOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 5 1 C1", "+ +"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "A1 5 1 C1", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);
	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 5 1 C1", "+ +"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "A1 5 1 C1", &mut Vec::new()),
		Err(PostfixError::NotEnoughOperands)
	);

	assert_eq!(
		Postfix::new(&mut Sheet {
			data: vec![vec!["5", "A1 A1 A1 A1 + + +"]],
			separator: ',',
		})
		.calc_cell(&Coord { column: 1, row: 0 }, "A1 A1 A1 A1 + + +", &mut Vec::new()),
		Ok(20.0)
	);
}

#[test]
fn process_sheet_test() {
	let data = vec![
		vec!["B1 B2 +", "2 B2 3 * -", "+"],
		vec!["A1", "5", "7 2 /"],
		vec!["C2 3 *", "1 2", "5 1 2 + 4 * + 3 -"],
	];
	let mut sheet = Sheet { data, separator: ',' };
	let postfix = Postfix::new(&mut sheet);
	let output = postfix.process_sheet(20);

	assert_eq!(output, String::from("-8,-13,#ERR\n-8,5,3.5\n10.5,#ERR,14"));
}
