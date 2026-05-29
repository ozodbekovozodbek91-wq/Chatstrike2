use tokio::net::TcpListener;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:7777").await?;
    info!("Chatstrike2 Server started on port 7777");

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("New client: {}", addr);

        tokio::spawn(async move {
            info!("Handling: {}", addr);
        });
    }
}
