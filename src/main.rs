use async_std::net;
use async_std::task;
use async_std::io::prelude::*;
use async_std::io::BufReader;
use async_std::stream::StreamExt;
use async_std::fs::File;
use async_std::io::ReadExt;
use async_std::io::WriteExt;

async fn write_file(mut socket: net::TcpStream, filename: &str) {
    let mut file = File::open(filename).await.unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).await.unwrap();
    socket.write(format!("{}\r\n\r\n", data).as_bytes()).await.unwrap();
}

async fn not_found(mut socket: net::TcpStream) {
    socket.write(b"Url Not Found\r\n\r\n").await.unwrap();
}

async fn serve(socket: net::TcpStream) {

    let mut clone = socket.clone();

    let mut buffer = BufReader::new(socket);
    let mut line = String::new();
    buffer.read_line(&mut line).await.unwrap();

    clone.write(b"HTTP/1.1 200 OK\r\n\r\n").await.unwrap();

    let url = line.split('\n').next().unwrap().to_string();
    match url.trim_end() {
        "GET / HTTP/1.1" => write_file(clone, "web/html/index.html").await,
        "GET /style HTTP/1.1" => write_file(clone, "web/css/style.css").await,
        "GET /app HTTP/1.1" => write_file(clone, "web/js/app.js").await,
        "GET /lib HTTP/1.1" => write_file(clone, "web/js/lib.js").await,
        _ => not_found(clone).await,
    };
}

fn main() {

    let address: String = "0.0.0.0:8000".to_string();

    task::block_on(async {
        let listener = net::TcpListener::bind(address).await;
        let new_connections = listener.unwrap();
        let mut incoming = new_connections.incoming();

        while let Some(socket_result) = incoming.next().await {
            let socket = socket_result.unwrap();
            task::spawn(async {
                serve(socket).await;
            });
        };
    })
}
