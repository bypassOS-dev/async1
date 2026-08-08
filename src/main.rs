use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
#[tokio::main]
async fn main() {
    let connect = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (mut client_socet, client_addr) = connect.accept().await.unwrap();

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
async fn handle_connect(client_socet: TcpStream, client_addr: String) {
    let server_socet = TcpStream::connect(&client_addr).await.unwrap();
    let (client_read, client_write) = client_socet.into_split();
    let (server_read, server_write) = server_socet.into_split();
}
