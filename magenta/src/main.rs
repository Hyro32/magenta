use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();
    println!("Hello from Magenta!");

    loop {
        let (_, address) = listener.accept().await.unwrap();
        println!("Accepted connection from {}", address);
    }
}
