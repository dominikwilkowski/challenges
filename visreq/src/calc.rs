pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
}

pub fn parse_line(line: &str) -> (Operator, i128) {
	let operator = match &line[0..1] {
		"+" => Operator::Add,
		"-" => Operator::Subtract,
		"*" | "x" => Operator::Multiply,
		"/" | ":" => Operator::Divide,
		_ => panic!("Invalid operator {}", &line[0..1]),
	};
	let operand = &line[1..].parse::<i128>().unwrap_or_else(|_| panic!("Number not valid {}", &line[1..]));

	(operator, *operand)
}

pub fn calc_lines(lines: Vec<String>) -> i128 {
	let mut result = 0;

	for line in lines {
		let (operator, operand) = parse_line(&line);
		match operator {
			Operator::Add => result += operand,
			Operator::Subtract => result -= operand,
			Operator::Multiply => result *= operand,
			Operator::Divide => {
				let sub = (result as f64 / operand as f64).floor();
				result = sub as i128;
			},
		}
	}

	result
}
