use std::env::args;

mod postfix;
mod shunting;

use crate::{postfix::Postfix, shunting::ShuntingYard};

fn main() {
	let expression = args().skip(1).collect::<Vec<String>>().join(" ");

	match ShuntingYard::infix_to_postfix(&expression) {
		Ok(ast) => match Postfix::calc(ast) {
			Ok(result) => println!("{result}"),
			Err(error) => println!("Error: {error:?}"),
		},
		Err(error) => println!("Error: {error:?}"),
	}
}
