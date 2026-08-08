use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
#[tokio::main]
async fn main() {
    let connect = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (mut socet, addr) = connect.accept().await.unwrap();

    }
}
async fn copy_data(mut from: TcpStream, mut to:TcpStream) {
    let mut buffer = [0u8; 1024];
    loop {
        let n = match from.read(&mut buffer).await  {
            Ok(0) => {
                println!("The connect is closet!");
                break;
            },
            Ok(n) => n,
            Err(e) => {
                println!("The error of read: {e}");
                break;
            },
        };
        if let Err(e) = to.write_all(&buffer[..n]).await {
            println!("The error of write!");
            break;
        }
        println!("Was sent {n} bytes");
    };
    
}
