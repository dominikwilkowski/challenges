extern crate rand;

use rand::Rng;

pub fn generate_random_expression(depth: usize) -> String {
	fn random_number() -> String {
		let mut rng = rand::thread_rng();
		rng.gen_range(0..10).to_string()
	}

	fn random_operator() -> char {
		let mut rng = rand::thread_rng();
		let operators = ['+', '-', '*', '/'];
		operators[rng.gen_range(0..operators.len())]
	}

	if depth == 0 {
		return random_number();
	}

	let left = generate_random_expression(depth - 1);
	let right = generate_random_expression(depth - 1);
	let operator = random_operator();

	format!("( {} {} {} )", left, operator, right)
}
