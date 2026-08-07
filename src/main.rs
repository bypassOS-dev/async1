use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}};
#[tokio::main]
async fn main() {
    let connect = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (mut socet, addr) = connect.accept().await.unwrap();

    }
}
async fn copy_data(mut from: TcpStream, to:TcpStream) {
    let mut buffer = [0u8; 1024];
    loop {
        let n = match from.read(&mut buffer).await  {
            Ok(0) => {

            },
            Ok(n) => {

            }
            Err(e) => {

            }
        };
    };
    
}
