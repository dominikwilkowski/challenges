use rand::RngExt;
use std::{
	sync::Arc,
	time::{Duration, Instant},
};
use tokio::{sync::Semaphore, task::JoinSet, time::sleep};

const MAX_CONCURRENT: usize = 5;

#[tokio::main]
async fn main() {
	let start_time = Instant::now();
	let permits = Arc::new(Semaphore::new(MAX_CONCURRENT));
	let mut set = JoinSet::new();
	for id in 1..=20 {
		let permits = permits.clone();
		set.spawn(worker(id, permits));
	}

	set.join_all().await;

	println!("\nDone in {}ms", start_time.elapsed().as_millis());
}

async fn worker(id: usize, permits: Arc<Semaphore>) -> Result<String, ()> {
	let Ok(_permit) = permits.acquire().await else {
		eprintln!("Task {} failed to acquire semaphore permit", id);
		return Err(());
	};

	let start_time = Instant::now();
	let in_flight = MAX_CONCURRENT - permits.available_permits();
	println!("Start ID [{id}] (in_flight: {in_flight})");

	let time = rand::rng().random_range(100..=500);
	sleep(Duration::from_millis(time)).await;

	println!("Finished ID [{id}] in {}ms", start_time.elapsed().as_millis());
	Ok(format!("Task {id} completed"))
}
