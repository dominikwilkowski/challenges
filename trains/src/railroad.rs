use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Clone)]
pub struct Route {
	destination: String,
	distance: u8,
}

#[derive(Debug, PartialEq)]
pub struct Station {
	routes: Vec<Route>,
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
	StationNotFound,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Trip {
	stops: Vec<String>,
	distance: u8,
}

#[derive(Debug, PartialEq)]
pub struct Railroad {
	stations: HashMap<String, Station>,
	stack: Vec<String>,
	visited: HashSet<String>,
	trips: Vec<Trip>,
}

impl Railroad {
	fn get_station_mut(&mut self, station_name: &str) -> Option<&mut Station> {
		self.stations.get_mut(station_name)
	}

	#[must_use]
	pub fn new() -> Self {
		Self {
			stations: HashMap::new(),
			stack: Vec::new(),
			visited: HashSet::new(),
			trips: Vec::new(),
		}
	}

	#[must_use = "Once routes are added, don't forget to calculate something with them"]
	pub fn add_route(mut self, from: &str, to: &str, distance: u8) -> Self {
		if let Some(start) = self.get_station_mut(from) {
			start.routes.push(Route {
				destination: to.to_string(),
				distance,
			});
		} else {
			let start = Station {
				routes: vec![Route {
					destination: to.to_string(),
					distance,
				}],
			};
			self.stations.insert(from.to_string(), start);
		}

		self
	}

	fn get_distance_from_to(&self, from: &str, to: &str) -> Result<u8, Error> {
		if let Some(station) = self.stations.get(from) {
			if let Some(route) = station.routes.iter().find(|route| route.destination == to) {
				Ok(route.distance)
			} else {
				Err(Error::StationNotFound)
			}
		} else {
			Err(Error::StationNotFound)
		}
	}

	pub fn get_distance(&self, route: Vec<&str>) -> u8 {
		if route.is_empty() {
			return 0;
		}

		let mut distance = 0;
		let mut from = route[0];

		for sub_route in route.iter().skip(1) {
			if let Ok(dis) = self.get_distance_from_to(from, sub_route) {
				distance += dis;
			} else {
				panic!("NO SUCH ROUTE");
			}
			from = sub_route;
		}

		distance
	}

	fn get_trips(&mut self, current_station: &str, to: &str, running_weight: u8) {
		if current_station == to && !self.stack.is_empty() {
			let stops = self.stack.clone();
			self.trips.push(Trip {
				stops: stops[..self.stack.len() - 1].to_vec(),
				distance: running_weight,
			});
		}

		if let Some(station) = self.stations.get(current_station) {
			for route in &station.routes.clone() {
				if !self.visited.contains(&route.destination) {
					self.stack.push(route.destination.to_string());
					self.visited.insert(route.destination.to_string());

					self.get_trips(&route.destination, to, running_weight + route.distance);

					self.stack.pop();
					self.visited.remove(&route.destination);
				}
			}
		}
	}

	pub fn get_trips_max_stops(&mut self, from: &str, to: &str, max_stops: u8) -> u8 {
		self.get_trips(from, to, 0);
		let mut trips = self.trips.clone();
		self.stack.clear();
		self.visited.clear();
		self.trips.clear();

		trips.retain(|trip| trip.stops.len() <= max_stops as usize);
		trips.len() as u8
	}

	pub fn get_trips_with_stops(&mut self, from: &str, to: &str, stops: u8) -> u8 {
		self.get_trips(from, to, 0);
		let mut trips = self.trips.clone();
		self.stack.clear();
		self.visited.clear();
		self.trips.clear();

		trips.retain(|trip| trip.stops.len() == stops as usize);
		trips.len() as u8
	}

	pub fn get_shortest_route_length(&mut self, from: &str, to: &str) -> u8 {
		let mut visited = HashSet::new();
		let mut que = VecDeque::new();

		que.push_back((from, 0));

		while let Some((current, distance)) = que.pop_front() {
			if let Some(station) = self.stations.get(current) {
				for route in &station.routes {
					let next = route.destination.as_str();
					if visited.insert(next) {
						if next == to {
							return distance + route.distance;
						}
						que.push_back((next, distance + route.distance));
					}
				}
			} else {
				panic!("NO SUCH ROUTE");
			}
		}

		panic!("NO SUCH ROUTE");
	}

	pub fn get_routes_max_distance(&mut self, from: &str, to: &str, max_distance: u8) -> u8 {
		self.get_trips(from, to, 0);
		let mut trips = self.trips.clone();
		self.stack.clear();
		self.visited.clear();
		self.trips.clear();

		trips.retain(|trip| trip.distance <= max_distance);
		trips.len() as u8
	}
}

#[cfg(test)]
mod testing {
	use super::*;

	#[test]
	fn new_test() {
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

		assert_eq!(railroad.stations.len(), 5);
		assert_eq!(
			railroad.stations.get("A"),
			Some(&Station {
				routes: vec![
					Route {
						destination: String::from("B"),
						distance: 5,
					},
					Route {
						destination: String::from("D"),
						distance: 5,
					},
					Route {
						destination: String::from("E"),
						distance: 7,
					},
				]
			})
		);

		assert_eq!(
			railroad.stations.get("B"),
			Some(&Station {
				routes: vec![Route {
					destination: String::from("C"),
					distance: 4,
				},]
			})
		);

		assert_eq!(
			railroad.stations.get("C"),
			Some(&Station {
				routes: vec![
					Route {
						destination: String::from("D"),
						distance: 8,
					},
					Route {
						destination: String::from("E"),
						distance: 2,
					},
				]
			})
		);

		assert_eq!(
			railroad.stations.get("D"),
			Some(&Station {
				routes: vec![
					Route {
						destination: String::from("C"),
						distance: 8,
					},
					Route {
						destination: String::from("E"),
						distance: 6,
					},
				]
			})
		);

		assert_eq!(
			railroad.stations.get("E"),
			Some(&Station {
				routes: vec![Route {
					destination: String::from("B"),
					distance: 3,
				},]
			})
		);
	}
}
