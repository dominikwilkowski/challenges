pub enum Operator {
	Add,
	Subtract,
	Multiply,
	Divide,
}

pub fn parse_line(line: &str) -> (Operator, i64) {
	let operator = match &line[0..1] {
		"+" => Operator::Add,
		"-" => Operator::Subtract,
		"*" | "x" => Operator::Multiply,
		"/" | ":" => Operator::Divide,
		_ => panic!("Invalid operator {}", &line[0..1]),
	};
	let operand = &line[1..]
		.replace(" ", "")
		.replace("q", "9")
		.replace("o", "0")
		.parse::<i64>()
		.expect(&format!("Number not valid {}", &line[1..].replace(" ", "").replace("q", "9").replace("o", "0")));

	(operator, *operand)
}

pub fn calc_lines(lines: Vec<String>) -> i64 {
	let mut result = 0;

	for line in lines {
		println!("{line}");
		let (operator, operand) = parse_line(&line);
		match operator {
			Operator::Add => result += operand,
			Operator::Subtract => result -= operand,
			Operator::Multiply => result *= operand,
			Operator::Divide => {
				let sub = (result as f64 / operand as f64).floor();
				result = sub as i64;
			},
		}
	}

	result
}
