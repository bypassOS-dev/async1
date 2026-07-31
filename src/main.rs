use std::time::{Instant, Duration};

async fn task_async(task: &str, seconds: u64) {
    println!("Start to do a {task}...");
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    println!("A task {task} was did!");
}
#[tokio::main]
async fn main() {
    let start = Instant::now();

    let a = tokio::spawn(task_async("Homework", 5));
    let b = tokio::spawn(task_async("Learn_rust", 4));

    a.await.unwrap();
    b.await.unwrap();

    println!("Time elapsed: {:?}", start.elapsed());
}