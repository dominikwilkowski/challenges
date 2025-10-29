use rand::Rng;
use std::fmt;

use process_csv::coord::Coord;

#[derive(Debug, PartialEq)]
pub enum Operator {
	Plus,
	Minus,
	Times,
	Divided,
}

impl fmt::Display for Operator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Operator::Plus => write!(f, "+"),
			Operator::Minus => write!(f, "-"),
			Operator::Times => write!(f, "*"),
			Operator::Divided => write!(f, "/"),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Token {
	Number(String),
	Operator(Operator),
	OpenParenthesis,
	CloseParenthesis,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Token::Number(number) => write!(f, "{number}"),
			Token::Operator(operator) => write!(f, "{operator}"),
			Token::OpenParenthesis => write!(f, "("),
			Token::CloseParenthesis => write!(f, ")"),
		}
	}
}

pub type Ast = Vec<Token>;

#[derive(Debug, PartialEq)]
pub enum PostfixError {
	UnknownCharacter,
}

pub struct Postfix;

impl Postfix {
	fn precedence(operator: &Token) -> u8 {
		match operator {
			Token::Operator(Operator::Plus) | Token::Operator(Operator::Minus) => 1,
			Token::Operator(Operator::Times) | Token::Operator(Operator::Divided) => 2,
			_ => 0,
		}
	}

	pub fn infix_to_postfix<R: Rng>(
		equation: &str,
		rows: usize,
		cols: usize,
		rng: &mut R,
	) -> Result<String, PostfixError> {
		let mut output_stack: Ast = Vec::new();
		let mut operator_stack: Ast = Vec::new();
		for token in Self::tokenize(equation)? {
			match token {
				Token::Number(_) => {
					output_stack.push(token);
				},
				Token::Operator(operator) => {
					let operator = Token::Operator(operator);
					while let Some(top_operator) = operator_stack.last() {
						if Self::precedence(&operator) <= Self::precedence(top_operator) {
							output_stack.push(operator_stack.pop().unwrap());
						} else {
							break;
						}
					}
					operator_stack.push(operator);
				},
				Token::OpenParenthesis => {
					operator_stack.push(token);
				},
				Token::CloseParenthesis => {
					while let Some(top_operator) = operator_stack.pop() {
						if top_operator == Token::OpenParenthesis {
							break;
						} else {
							output_stack.push(top_operator);
						}
					}
				},
			}
		}

		while let Some(top_operator) = operator_stack.pop() {
			output_stack.push(top_operator);
		}

		Ok(
			output_stack
				.iter()
				.map(|item| match item {
					Token::Number(_) => {
						if rng.gen_range(0..40) == 7 {
							Coord {
								column: rng.gen_range(1..=(cols - 1)),
								row: rng.gen_range(1..=(rows - 1)),
							}
							.stringify()
						} else {
							item.to_string()
						}
					},
					_ => item.to_string(),
				})
				.collect::<Vec<String>>()
				.join(" "),
		)
	}

	fn tokenize(equation: &str) -> Result<Ast, PostfixError> {
		let mut tokens = Vec::new();
		let mut items = equation.chars().peekable();

		while let Some(item) = items.peek() {
			match item {
				'0'..='9' | '.' => {
					let mut number = String::new();
					while let Some(&next_item) = items.peek() {
						if next_item.is_ascii_digit() || next_item == '.' {
							number.push(next_item);
							items.next();
						} else {
							break;
						}
					}
					tokens.push(Token::Number(number));
				},
				'+' => {
					tokens.push(Token::Operator(Operator::Plus));
					items.next();
				},
				'-' => {
					tokens.push(Token::Operator(Operator::Minus));
					items.next();
				},
				'*' => {
					tokens.push(Token::Operator(Operator::Times));
					items.next();
				},
				'/' => {
					tokens.push(Token::Operator(Operator::Divided));
					items.next();
				},
				'(' => {
					tokens.push(Token::OpenParenthesis);
					items.next();
				},
				')' => {
					tokens.push(Token::CloseParenthesis);
					items.next();
				},
				' ' => {
					items.next();
				},
				_ => {
					return Err(PostfixError::UnknownCharacter);
				},
			}
		}

		Ok(tokens)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::{rngs::StdRng, SeedableRng};

	#[test]
	fn tokenize_test() {
		assert_eq!(
			Postfix::tokenize("1 +2"),
			Ok(vec![
				Token::Number(String::from("1")),
				Token::Operator(Operator::Plus),
				Token::Number(String::from("2"))
			])
		);
		assert_eq!(
			Postfix::tokenize("12 +2"),
			Ok(vec![
				Token::Number(String::from("12")),
				Token::Operator(Operator::Plus),
				Token::Number(String::from("2"))
			])
		);
		assert_eq!(
			Postfix::tokenize("999 / 666"),
			Ok(vec![
				Token::Number(String::from("999")),
				Token::Operator(Operator::Divided),
				Token::Number(String::from("666"))
			])
		);
		assert_eq!(
			Postfix::tokenize("9+(2*4)-80/0"),
			Ok(vec![
				Token::Number(String::from("9")),
				Token::Operator(Operator::Plus),
				Token::OpenParenthesis,
				Token::Number(String::from("2")),
				Token::Operator(Operator::Times),
				Token::Number(String::from("4")),
				Token::CloseParenthesis,
				Token::Operator(Operator::Minus),
				Token::Number(String::from("80")),
				Token::Operator(Operator::Divided),
				Token::Number(String::from("0"))
			])
		);
	}

	#[test]
	fn infix_to_postfix_test() {
		let seed = [0u8; 32];
		let mut rng: StdRng = SeedableRng::from_seed(seed);

		assert_eq!(Postfix::infix_to_postfix("1 +2", 10, 10, &mut rng), Ok(String::from("1 2 +")));
		assert_eq!(Postfix::infix_to_postfix("1+  2", 10, 10, &mut rng), Ok(String::from("1 2 +")));
		assert_eq!(Postfix::infix_to_postfix(" 3 + 4 *  ( 2 - 10) ", 10, 10, &mut rng), Ok(String::from("3 4 2 D8 - * +")));
	}
}
