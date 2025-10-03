use std::fmt;

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
	Number(f64),
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

#[derive(Debug, PartialEq)]
pub enum ShuntingYardError {
	ItemNotRecognized,
}

pub type Ast = Vec<Token>;

pub struct ShuntingYard {}

impl ShuntingYard {
	fn precedence(token: &Token) -> u8 {
		match token {
			Token::Operator(Operator::Plus) | Token::Operator(Operator::Minus) => 1,
			Token::Operator(Operator::Times) | Token::Operator(Operator::Divided) => 2,
			_ => 0,
		}
	}

	pub fn infix_to_postfix(expression: &str) -> Result<Ast, ShuntingYardError> {
		let mut output_stack: Ast = Vec::new();
		let mut operator_stack: Ast = Vec::new();

		for token in Self::tokenize(expression)? {
			match token {
				Token::Number(_) => output_stack.push(token),
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

		Ok(output_stack)
	}

	fn tokenize(expression: &str) -> Result<Ast, ShuntingYardError> {
		let mut tokens = Vec::new();
		let mut items = expression.chars().peekable();

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
					tokens.push(Token::Number(number.parse().unwrap()));
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
					// skipping whitespace
					items.next();
				},
				_ => return Err(ShuntingYardError::ItemNotRecognized),
			}
		}

		Ok(tokens)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn tokenize_test() {
		assert_eq!(
			ShuntingYard::tokenize("1 +2"),
			Ok(vec![Token::Number(1.0), Token::Operator(Operator::Plus), Token::Number(2.0)])
		);
		assert_eq!(
			ShuntingYard::tokenize("12 +2"),
			Ok(vec![Token::Number(12.0), Token::Operator(Operator::Plus), Token::Number(2.0)])
		);
		assert_eq!(
			ShuntingYard::tokenize("999 / 666"),
			Ok(vec![
				Token::Number(999.0),
				Token::Operator(Operator::Divided),
				Token::Number(666.0)
			])
		);
		assert_eq!(
			ShuntingYard::tokenize("9+(2*4)-80/0"),
			Ok(vec![
				Token::Number(9.0),
				Token::Operator(Operator::Plus),
				Token::OpenParenthesis,
				Token::Number(2.0),
				Token::Operator(Operator::Times),
				Token::Number(4.0),
				Token::CloseParenthesis,
				Token::Operator(Operator::Minus),
				Token::Number(80.0),
				Token::Operator(Operator::Divided),
				Token::Number(0.0)
			])
		);
	}

	#[test]
	fn infix_to_postfix_test() {
		assert_eq!(
			ShuntingYard::infix_to_postfix("1 +2"),
			Ok(vec![Token::Number(1.0), Token::Number(2.0), Token::Operator(Operator::Plus)])
		);
		assert_eq!(
			ShuntingYard::infix_to_postfix("1+  2"),
			Ok(vec![Token::Number(1.0), Token::Number(2.0), Token::Operator(Operator::Plus)])
		);
		assert_eq!(
			ShuntingYard::infix_to_postfix(" 3 + 4 *  ( 2 - 10) "),
			Ok(vec![
				Token::Number(3.0),
				Token::Number(4.0),
				Token::Number(2.0),
				Token::Number(10.0),
				Token::Operator(Operator::Minus),
				Token::Operator(Operator::Times),
				Token::Operator(Operator::Plus),
			])
		);
	}
}
