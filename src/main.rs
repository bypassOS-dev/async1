use thiserror::Error;
use tokio::net::TcpStream;
use tokio;
#[derive(Debug, Error)]
enum MyError {
    #[error("[!!!]Error of network")]
    Network,
    #[error("[!!!]Error of Timeout")]
    TimeOut,
    #[error("[!!!]Error of data")]
    InvalidData,
}
async fn main() {
    let conect = TcpStream::connect("127.0.0.1:9999")
}
