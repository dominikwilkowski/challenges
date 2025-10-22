pub struct Railroad {/* TODO: add fields */}

impl Railroad {
	#[must_use]
	pub fn new() -> Self {
		Self {}
	}

	#[must_use = "Once routes are added don't forget to calculate something with them"]
	pub fn add_route(mut self, _from: &str, _to: &str, _distance: u8) -> Self {
		// TODO: add them routes already!
		self
	}

	pub fn get_distance(&self, _route: Vec<&str>) -> u8 {
		todo!("Calculate distance");
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

	pub fn get_number_routes_max_distance(&self, _from: &str, _to: &str, _max_distance: u8) -> u8 {
		todo!("Find how many routes");
	}
}
