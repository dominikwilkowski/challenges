use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Railroad {
	stations: HashMap<String, Station>,
}

impl Railroad {
	fn get_station(&self, station_name: &str) -> Option<&Station> {
		self.stations.get(station_name)
	}

	fn get_station_mut(&mut self, station_name: &str) -> Option<&mut Station> {
		self.stations.get_mut(station_name)
	}

	#[must_use]
	pub fn new() -> Self {
		Self {
			stations: HashMap::new(),
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

		for i in 1..route.len() {
			if let Ok(dis) = self.get_distance_from_to(from, route[i]) {
				distance += dis;
			} else {
				panic!("NO SUCH ROUTE");
			}
			from = route[i];
		}

		distance
	}

	pub fn get_routes_max_stops(&self, _from: &str, _to: &str, _max_stops: u8) -> u8 {
		todo!("Find number of trips with max stops");
	}

	pub fn get_routes_with_stops(&self, _from: &str, _to: &str, _stops: u8) -> u8 {
		todo!("Find number of trips with exact stops");
	}

	pub fn get_shortest_route_length(&self, _from: &str, _to: &str) -> u8 {
		todo!("Find shortest route");
	}

	pub fn get_routes_max_distance(&self, _from: &str, _to: &str, _max_distance: u8) -> u8 {
		todo!("Find how many routes");
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
