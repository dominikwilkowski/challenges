enum Direction {
	North,
	West,
	East,
	South,
}

#[derive(Debug, PartialEq)]
struct Size(u8, u8);

#[derive(Debug, PartialEq)]
struct Position(u8, u8);

#[derive(Debug)]
struct Wearhouse {
	size: Size,
	position: Position,
}

impl Wearhouse {
	fn new(size: Size, position: Position) -> Self {
		Self { position, size }
	}

	fn visualize(&self) -> String {
		let mut output = format!("┌{:─^width$}┐\n", "", width = self.size.0.into());
		for row in 0..self.size.1 {
			output.push('│');
			for col in 0..self.size.0 {
				if self.position.0 == row && self.position.1 == col {
					output.push('X');
				} else {
					output.push(' ');
				}
			}
			output.push_str("│\n");
		}
		output.push_str(&format!("└{:─^width$}┘\n", "─", width = self.size.0.into()));
		output.push_str(&format!("({} {})\n", self.position.0, self.position.1));

		output
	}

	fn moving(&mut self, dir: Direction) {
		match dir {
			Direction::North => {
				if self.position.0 != 0 {
					self.position.0 -= 1;
				}
			}
			Direction::West => {
				if self.position.1 != 0 {
					self.position.1 -= 1;
				}
			}
			Direction::East => {
				if self.position.1 != self.size.1 - 1 {
					self.position.1 += 1;
				}
			}
			Direction::South => {
				if self.position.0 != self.size.0 - 1 {
					self.position.0 += 1;
				}
			}
		}
	}

	fn batch_moving(&mut self, instructions: &str) {
		instructions.split_whitespace().for_each(|movement| match movement.to_uppercase().as_str() {
			"N" => self.moving(Direction::North),
			"W" => self.moving(Direction::West),
			"E" => self.moving(Direction::East),
			"S" => self.moving(Direction::South),
			_ => {}
		});
	}
}

fn main() {
	let mut my_wearhouse = Wearhouse::new(Size(10, 10), Position(9, 0));
	println!("{}", my_wearhouse.visualize());
	my_wearhouse.batch_moving("N E N E N E N E");
	println!("{}", my_wearhouse.visualize());
}

#[cfg(test)]
mod testing {
	use super::*;

	#[test]
	fn setting_size() {
		let my_wearhouse = Wearhouse::new(Size(10, 10), Position(0, 0));
		assert_eq!(my_wearhouse.size, Size(10, 10));

		let my_wearhouse = Wearhouse::new(Size(12, 12), Position(0, 0));
		assert_eq!(my_wearhouse.size, Size(12, 12));

		let my_wearhouse = Wearhouse::new(Size(7, 14), Position(0, 0));
		assert_eq!(my_wearhouse.size, Size(7, 14));
	}

	#[test]
	fn moving_north() {
		let mut my_wearhouse = Wearhouse::new(Size(10, 10), Position(5, 5));
		my_wearhouse.moving(Direction::North);
		assert_eq!(my_wearhouse.position, Position(4, 5));
		my_wearhouse.moving(Direction::North);
		assert_eq!(my_wearhouse.position, Position(3, 5));
		my_wearhouse.moving(Direction::North);
		assert_eq!(my_wearhouse.position, Position(2, 5));
		my_wearhouse.moving(Direction::North);
		assert_eq!(my_wearhouse.position, Position(1, 5));
		my_wearhouse.moving(Direction::North);
		assert_eq!(my_wearhouse.position, Position(0, 5));
		my_wearhouse.moving(Direction::North);
		assert_eq!(my_wearhouse.position, Position(0, 5));
		my_wearhouse.moving(Direction::North);
		assert_eq!(my_wearhouse.position, Position(0, 5));
	}

	#[test]
	fn moving_south() {
		let mut my_wearhouse = Wearhouse::new(Size(10, 10), Position(5, 5));
		my_wearhouse.moving(Direction::South);
		assert_eq!(my_wearhouse.position, Position(6, 5));
		my_wearhouse.moving(Direction::South);
		assert_eq!(my_wearhouse.position, Position(7, 5));
		my_wearhouse.moving(Direction::South);
		assert_eq!(my_wearhouse.position, Position(8, 5));
		my_wearhouse.moving(Direction::South);
		assert_eq!(my_wearhouse.position, Position(9, 5));
		my_wearhouse.moving(Direction::South);
		assert_eq!(my_wearhouse.position, Position(9, 5));
		my_wearhouse.moving(Direction::South);
		assert_eq!(my_wearhouse.position, Position(9, 5));
		my_wearhouse.moving(Direction::South);
		assert_eq!(my_wearhouse.position, Position(9, 5));
	}

	#[test]
	fn moving_west() {
		let mut my_wearhouse = Wearhouse::new(Size(10, 10), Position(5, 5));
		my_wearhouse.moving(Direction::West);
		assert_eq!(my_wearhouse.position, Position(5, 4));
		my_wearhouse.moving(Direction::West);
		assert_eq!(my_wearhouse.position, Position(5, 3));
		my_wearhouse.moving(Direction::West);
		assert_eq!(my_wearhouse.position, Position(5, 2));
		my_wearhouse.moving(Direction::West);
		assert_eq!(my_wearhouse.position, Position(5, 1));
		my_wearhouse.moving(Direction::West);
		assert_eq!(my_wearhouse.position, Position(5, 0));
		my_wearhouse.moving(Direction::West);
		assert_eq!(my_wearhouse.position, Position(5, 0));
		my_wearhouse.moving(Direction::West);
		assert_eq!(my_wearhouse.position, Position(5, 0));
	}

	#[test]
	fn moving_east() {
		let mut my_wearhouse = Wearhouse::new(Size(10, 10), Position(5, 5));
		my_wearhouse.moving(Direction::East);
		assert_eq!(my_wearhouse.position, Position(5, 6));
		my_wearhouse.moving(Direction::East);
		assert_eq!(my_wearhouse.position, Position(5, 7));
		my_wearhouse.moving(Direction::East);
		assert_eq!(my_wearhouse.position, Position(5, 8));
		my_wearhouse.moving(Direction::East);
		assert_eq!(my_wearhouse.position, Position(5, 9));
		my_wearhouse.moving(Direction::East);
		assert_eq!(my_wearhouse.position, Position(5, 9));
		my_wearhouse.moving(Direction::East);
		assert_eq!(my_wearhouse.position, Position(5, 9));
		my_wearhouse.moving(Direction::East);
		assert_eq!(my_wearhouse.position, Position(5, 9));
	}

	#[test]
	fn batching() {
		let mut my_wearhouse = Wearhouse::new(Size(10, 10), Position(9, 0));
		my_wearhouse.batch_moving("N E N E  N E N E");
		assert_eq!(my_wearhouse.position, Position(5, 4));
	}
}
