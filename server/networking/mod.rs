use tokio::{
    net::TcpListener,
    io::{AsyncReadExt, AsyncWriteExt}
};

pub async fn listen_for_client() {
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    
    let (mut socket, _addr) = listener.accept().await.unwrap();
    loop{

        let mut buffer = [0u8; 1024];
        
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}