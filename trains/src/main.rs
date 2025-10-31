mod railroad;

use railroad::Railroad;

fn main() {
	let mut railroad = Railroad::new()
		.add_route("A", "B", 5)
		.add_route("B", "C", 4)
		.add_route("C", "D", 8)
		.add_route("D", "C", 8)
		.add_route("D", "E", 6)
		.add_route("A", "D", 5)
		.add_route("C", "E", 2)
		.add_route("E", "B", 3)
		.add_route("A", "E", 7);

	// 1. The distance of the route A-B-C.
	// 2. The distance of the route A-D.
	// 3. The distance of the route A-D-C.
	// 4. The distance of the route A-E-B-C-D.
	// 5. The distance of the route A-E-D.
	debug_assert_eq!(railroad.get_distance(vec!["A", "B", "C"]), 9);
	debug_assert_eq!(railroad.get_distance(vec!["A", "D"]), 5);
	debug_assert_eq!(railroad.get_distance(vec!["A", "D", "C"]), 13);
	debug_assert_eq!(railroad.get_distance(vec!["A", "E", "B", "C", "D"]), 22);
	// should_panic: debug_assert_eq!(railroad.get_distance(vec!["A", "E", "D"]), 13);

	// 6. The number of trips starting at C and ending at C with a maximum of 3 stops.
	debug_assert_eq!(railroad.get_trips_max_stops("C", "C", 3), 3);

	// 7. The number of trips starting at A and ending at C with exactly 4 stops.
	debug_assert_eq!(railroad.get_trips_with_stops("A", "C", 3), 1);

	// 8. The length of the shortest route (in terms of distance to travel) from A to C.
	// 9. The length of the shortest route (in terms of distance to travel) from B to B.
	debug_assert_eq!(railroad.get_shortest_route_length("A", "C"), 9);
	debug_assert_eq!(railroad.get_shortest_route_length("B", "B"), 9);

	// 10. The number of different routes from C to C with a distance of less than 30.
	debug_assert_eq!(railroad.get_routes_max_distance("C", "C", 30), 3);
}

#[cfg(test)]
mod testing {
	use super::*;

	#[test]
	fn get_distance_test() {
		let railroad = Railroad::new()
			.add_route("A", "B", 5)
			.add_route("B", "C", 4)
			.add_route("C", "D", 8)
			.add_route("D", "C", 8)
			.add_route("D", "E", 6)
			.add_route("A", "D", 5)
			.add_route("C", "E", 2)
			.add_route("E", "B", 3)
			.add_route("A", "E", 7);

		assert_eq!(railroad.get_distance(vec!["A", "B", "C"]), 9);
		assert_eq!(railroad.get_distance(vec!["A", "D"]), 5);
		assert_eq!(railroad.get_distance(vec!["A", "D", "C"]), 13);
		assert_eq!(railroad.get_distance(vec!["A", "E", "B", "C", "D"]), 22);
	}

	#[should_panic]
	#[test]
	fn get_distance_missing() {
		let railroad = Railroad::new()
			.add_route("A", "B", 5)
			.add_route("B", "C", 4)
			.add_route("C", "D", 8)
			.add_route("D", "C", 8)
			.add_route("D", "E", 6)
			.add_route("A", "D", 5)
			.add_route("C", "E", 2)
			.add_route("E", "B", 3)
			.add_route("A", "E", 7);

		railroad.get_distance(vec!["A", "E", "D"]);
	}

	#[test]
	fn get_routes_max_stops_test() {
		let mut railroad = Railroad::new()
			.add_route("A", "B", 5)
			.add_route("B", "C", 4)
			.add_route("C", "D", 8)
			.add_route("D", "C", 8)
			.add_route("D", "E", 6)
			.add_route("A", "D", 5)
			.add_route("C", "E", 2)
			.add_route("E", "B", 3)
			.add_route("A", "E", 7);

		assert_eq!(railroad.get_trips_max_stops("C", "C", 3), 3);
	}

	#[test]
	fn get_routes_with_stops_test() {
		let mut railroad = Railroad::new()
			.add_route("A", "B", 5)
			.add_route("B", "C", 4)
			.add_route("C", "D", 8)
			.add_route("D", "C", 8)
			.add_route("D", "E", 6)
			.add_route("A", "D", 5)
			.add_route("C", "E", 2)
			.add_route("E", "B", 3)
			.add_route("A", "E", 7);

		assert_eq!(railroad.get_trips_with_stops("A", "C", 3), 1);
	}

	#[test]
	fn get_shortest_route_length_test() {
		let mut railroad = Railroad::new()
			.add_route("A", "B", 5)
			.add_route("B", "C", 4)
			.add_route("C", "D", 8)
			.add_route("D", "C", 8)
			.add_route("D", "E", 6)
			.add_route("A", "D", 5)
			.add_route("C", "E", 2)
			.add_route("E", "B", 3)
			.add_route("A", "E", 7);

		assert_eq!(railroad.get_shortest_route_length("A", "C"), 9);
		assert_eq!(railroad.get_shortest_route_length("B", "B"), 9);
	}

	#[test]
	fn get_routes_max_distance_test() {
		let mut railroad = Railroad::new()
			.add_route("A", "B", 5)
			.add_route("B", "C", 4)
			.add_route("C", "D", 8)
			.add_route("D", "C", 8)
			.add_route("D", "E", 6)
			.add_route("A", "D", 5)
			.add_route("C", "E", 2)
			.add_route("E", "B", 3)
			.add_route("A", "E", 7);

		assert_eq!(railroad.get_routes_max_distance("C", "C", 30), 3);
	}
}
