use crate::shunting::{Ast, Operator, Token};

#[derive(Debug, PartialEq)]
pub enum PostfixError {
	TooManyOperands,
	NotEnoughOperands,
	DivisionByZero,
}

#[derive(Debug, PartialEq)]
pub struct Postfix {}

impl Postfix {
	pub fn calc(equation: Ast) -> Result<f64, PostfixError> {
		let mut stack = Vec::new();

		for item in equation {
			match item {
				Token::Number(number) => {
					stack.push(number);
				},
				Token::Operator(Operator::Plus) => {
					if stack.len() < 2 {
						return Err(PostfixError::NotEnoughOperands);
					}
					let (operand_b, operand_a) = (stack.pop(), stack.pop());
					stack.push(operand_a.unwrap() + operand_b.unwrap());
				},
				Token::Operator(Operator::Minus) => {
					if stack.len() < 2 {
						return Err(PostfixError::NotEnoughOperands);
					}
					let (operand_b, operand_a) = (stack.pop(), stack.pop());
					stack.push(operand_a.unwrap() - operand_b.unwrap());
				},
				Token::Operator(Operator::Times) => {
					if stack.len() < 2 {
						return Err(PostfixError::NotEnoughOperands);
					}
					let (operand_b, operand_a) = (stack.pop(), stack.pop());
					stack.push(operand_a.unwrap() * operand_b.unwrap());
				},
				Token::Operator(Operator::Divided) => {
					if stack.len() < 2 {
						return Err(PostfixError::NotEnoughOperands);
					}
					let (operand_b, operand_a) = (stack.pop(), stack.pop());
					if operand_b == Some(0.0) {
						return Err(PostfixError::DivisionByZero);
					}
					stack.push(operand_a.unwrap() / operand_b.unwrap());
				},
				Token::OpenParenthesis | Token::CloseParenthesis => {
					unreachable!("Postfix notations should not contain parenthesis")
				},
			}
		}

		if stack.len() > 1 {
			Err(PostfixError::TooManyOperands)
		} else {
			Ok(stack[0])
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn calc_test() {
		assert_eq!(Postfix::calc(vec![Token::Number(5.0), Token::Number(10.0), Token::Operator(Operator::Plus)]), Ok(15.0));
		assert_eq!(
			Postfix::calc(vec![
				Token::Number(5.0),
				Token::Number(10.0),
				Token::Operator(Operator::Minus)
			]),
			Ok(-5.0)
		);
		assert_eq!(
			Postfix::calc(vec![
				Token::Number(5.0),
				Token::Number(10.0),
				Token::Operator(Operator::Times)
			]),
			Ok(50.0)
		);
		assert_eq!(
			Postfix::calc(vec![
				Token::Number(5.0),
				Token::Number(10.0),
				Token::Operator(Operator::Divided)
			]),
			Ok(0.5)
		);
		assert_eq!(Postfix::calc(vec![Token::Number(5.0)]), Ok(5.0));
		assert_eq!(
			Postfix::calc(vec![
				Token::Number(3.0),
				Token::Number(4.0),
				Token::Number(2.0),
				Token::Number(10.0),
				Token::Operator(Operator::Minus),
				Token::Operator(Operator::Times),
				Token::Operator(Operator::Plus),
			]),
			Ok(-29.0)
		);

		assert_eq!(
			Postfix::calc(vec![Token::Number(10.0), Token::Operator(Operator::Plus)]),
			Err(PostfixError::NotEnoughOperands)
		);
		assert_eq!(
			Postfix::calc(vec![
				Token::Number(5.0),
				Token::Number(10.0),
				Token::Operator(Operator::Plus),
				Token::Number(7.0)
			]),
			Err(PostfixError::TooManyOperands)
		);
		assert_eq!(
			Postfix::calc(vec![
				Token::Number(5.0),
				Token::Number(0.0),
				Token::Operator(Operator::Divided)
			]),
			Err(PostfixError::DivisionByZero)
		);
	}
}
