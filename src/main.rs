// Uncomment this block to pass the first stage
use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt}  // allows stream.write() and stream.read() to be async
};

async fn process(stream: &mut TcpStream) {
    loop {
        let mut buf: [u8; 512] = [0; 512];
        let read_count = stream.read(&mut buf).await.unwrap();
        
        if read_count == 0 {
            return;
        }
        stream.write(b"+PONG\r\n").await.unwrap();
    }
}


#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let stream: Result<(tokio::net::TcpStream, std::net::SocketAddr), std::io::Error> = listener.accept().await;
        match stream {
            Ok((mut stream, _)) => {
                // move stream into newly spawned task
                tokio::spawn(async move {
                    process(&mut stream).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
