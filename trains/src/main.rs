mod railroad;

use railroad::Railroad;

fn main() {
	println!("Hello, world!");
}

#[cfg(test)]
mod testing {
	use super::*;

	#[test]
	fn input_test() {
		let railroad = Railroad::new()
			.route("A", "B", 5)
			.route("B", "C", 4)
			.route("C", "D", 8)
			.route("D", "C", 8)
			.route("D", "E", 6)
			.route("A", "D", 5)
			.route("C", "E", 2)
			.route("E", "B", 3)
			.route("A", "E", 7);

		// assert routes inputted
	}

	#[test]
	fn get_distance_test() {
		let railroad = Railroad::new()
			.route("A", "B", 5)
			.route("B", "C", 4)
			.route("C", "D", 8)
			.route("D", "C", 8)
			.route("D", "E", 6)
			.route("A", "D", 5)
			.route("C", "E", 2)
			.route("E", "B", 3)
			.route("A", "E", 7);

		assert_eq!(railroad.get_distance(vec!["A", "B", "C"]), 9);
		assert_eq!(railroad.get_distance(vec!["A", "D"]), 5);
		assert_eq!(railroad.get_distance(vec!["A", "D", "C"]), 13);
		assert_eq!(railroad.get_distance(vec!["A", "E", "B", "C", "D"]), 22);
		assert_eq!(railroad.get_distance(vec!["A", "E", "D"]), 13);
	}
}
