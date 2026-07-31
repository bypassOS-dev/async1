use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    println!("Start working...");

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        for i in 1..=5 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let result = i * 10;
            tx.send(result).await.unwrap();
            println!("a programm send a data!");
        }
    });
    while let Some(value) = rx.recv().await {
        println!("A main function is geting a data: {value}");
    }
}
