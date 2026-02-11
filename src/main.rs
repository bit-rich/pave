use tokio::net::UdpSocket;
#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("[::]:9999").await.unwrap();
    let mut buffer: [u8; 1024] = [0;1024];
    let (byte_count,address) = socket.recv_from(&mut buffer).await.unwrap();
    println!("Recieved {byte_count} bytes from {address}");
}
