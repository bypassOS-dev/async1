use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Proxy слушает на 127.0.0.1:8080");

    // Адрес, куда пересылаем всё, что приходит на 8080.
    // Для теста подними отдельно свой эхо-сервер (например, с Дня 8) на 9090.
    let target_addr = "127.0.0.1:9090".to_string();

    loop {
        let (client_socket, client_addr) = listener.accept().await.unwrap();
        println!("Новый клиент: {client_addr}");

        let target = target_addr.clone();
        tokio::spawn(async move {
            handle_connect(client_socket, target).await;
        });
    }
}

async fn handle_connect(client_socket: TcpStream, target_addr: String) {
    let server_socket = match TcpStream::connect(&target_addr).await {
        Ok(s) => s,
        Err(e) => {
            println!("Не удалось подключиться к {target_addr}: {e}");
            return;
        }
    };
    println!("Подключились к целевому серверу {target_addr}");

    let (client_read, client_write) = client_socket.into_split();
    let (server_read, server_write) = server_socket.into_split();

    let client_to_server = tokio::spawn(copy_data(
        client_read,
        server_write,
        "клиент→сервер".to_string(),
    ));
    let server_to_client = tokio::spawn(copy_data(
        server_read,
        client_write,
        "сервер→клиент".to_string(),
    ));

    let _ = tokio::join!(client_to_server, server_to_client);

    println!("Проксирование завершено.");
}

async fn copy_data(
    mut from: tokio::net::tcp::OwnedReadHalf,
    mut to: tokio::net::tcp::OwnedWriteHalf,
    label: String,
) {
    let mut buffer = [0u8; 1024];

    loop {
        let n = match from.read(&mut buffer).await {
            Ok(0) => {
                println!("{label}: соединение закрыто");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                println!("{label}: ошибка чтения: {e}");
                break;
            }
        };

        if let Err(e) = to.write_all(&buffer[..n]).await {
            println!("{label}: ошибка записи: {e}");
            break;
        }

        println!("{label}: переслано {n} байт");
    }
}
