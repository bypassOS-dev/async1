use std::time::{Instant, Duration};

use tokio::join;

async fn task_async(task: &str, seconds: u64) {
    println!("Start to do a {task}...");
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    println!("A task {task} was did!");
}
#[tokio::main]
async fn main() {
    let start = Instant::now();

    join!(task_async("homework", 2), task_async("Rust", 3));

    println!("Time elapsed: {:?}", start.elapsed());
}
