pub struct Railroad {}

impl Railroad {
	pub fn new() -> Self {
		Self {}
	}

	pub fn route(&mut self, from: &str, to: &str, distance: u8) -> Self {
		todo!("Add routes");
		self
	}

	pub fn get_distance(&self, route: Vec<&str>) -> u8 {
		todo!("Calculate distance");
		0
	}

	pub fn get_routes_max_stops(&self, from: &str, to: &str, max_stops: u8) -> u8 {
		todo!("Find number of trips with max stops");
		0
	}

	pub fn get_routes_with_stops(&self, from: &str, to: &str, stops: u8) -> u8 {
		todo!("Find number of trips with exact stops");
		0
	}

	pub fn get_shortest_route_length(&self, from: &str, to: &str) -> u8 {
		todo!("Find shortest route");
		0
	}

	pub fn get_shortest_routes(&self, from: &str, to: &str) -> u8 {
		todo!("Find how many routes");
		0
	}
}
