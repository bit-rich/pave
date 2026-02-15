use tokio::net::UdpSocket;
use std::fs::read_to_string;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let ip = read_to_string("peers.txt").unwrap().trim().to_string();
    let socket = UdpSocket::bind("[::]:9999").await.unwrap();
    let sender_socket = Arc::new(socket);
    let receiver_socket = sender_socket.clone();
    tokio::spawn(async move {
        let mut buffer: [u8; 1024] = [0;1024];
        loop{
            let (byte_count,address) = receiver_socket.recv_from(&mut buffer).await.unwrap();
            println!("Recieved {byte_count} bytes from {address}");
        }
    });
    loop{
        sleep(Duration::from_secs(10)).await;
        sender_socket.send_to(b"hello",&ip).await.unwrap();
    }


}
