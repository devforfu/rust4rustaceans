use async_std::{net::{self, ToSocketAddrs}, task};
use futures::{StreamExt, executor::block_on};

fn main() {
    block_on(async {
        if let Err(_) = listen().await {
            println!("Error");
        }
    });
}

async fn listen() -> std::io::Result<()> {
    let addr = "localhost:8080";
    let listener = net::TcpListener::bind(addr).await?;
    let mut new_connections = listener.incoming();
    while let Some(socket_result) = new_connections.next().await {
        let socket = socket_result?;
        task::spawn(async {
            serve(socket).await;
        });
    }
    Ok(())
}

async fn serve(socket: net::TcpStream) {
    println!("Connection from {}", socket.peer_addr().unwrap());
}
