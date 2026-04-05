use rand::RngExt;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
	let worker = worker(1).await;
	println!("{worker}");
}

async fn worker(id: usize) -> String {
	let time = rand::rng().random_range(100..=500);
	sleep(Duration::from_millis(time)).await;
	format!("Task {id} completed")
}
