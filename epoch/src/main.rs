use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let input = input.trim();
	let mut output = String::new();

	for num_str in input.lines() {
		if let Ok(num) = num_str.parse::<i64>() {
			output.push_str(&format!("{:?}", get_day_after_epoch(num)));
			output.push('\n');
		} else {
			println!("Can't parse input {num_str}");
		}
	}

	println!("{}", output.trim());
}

#[derive(Debug, PartialEq)]
enum DayOfWeek {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
	Sunday,
}

fn get_day_after_epoch(days: i64) -> DayOfWeek {
	match days.rem_euclid(7) as u8 {
		0 => DayOfWeek::Thursday,
		1 => DayOfWeek::Friday,
		2 => DayOfWeek::Saturday,
		3 => DayOfWeek::Sunday,
		4 => DayOfWeek::Monday,
		5 => DayOfWeek::Tuesday,
		_ => DayOfWeek::Wednesday,
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn get_day_after_epoch_test() {
		assert_eq!(get_day_after_epoch(100), DayOfWeek::Saturday);
		assert_eq!(get_day_after_epoch(0), DayOfWeek::Thursday);
		assert_eq!(get_day_after_epoch(128), DayOfWeek::Saturday);
		assert_eq!(get_day_after_epoch(2544), DayOfWeek::Sunday);
		assert_eq!(get_day_after_epoch(-5932), DayOfWeek::Monday);
	}
}
