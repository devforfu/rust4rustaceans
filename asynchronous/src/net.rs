use async_std::stream::StreamExt;
use async_std::{net, task};

pub struct Listener {
    addr: String,
}

impl Listener {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    pub async fn listen_forever(&self) -> std::io::Result<()> {
        let (_, tx) = async_std::channel::unbounded::<()>();
        listen(tx).await
    }

    pub async fn listen_until(
        &self,
        event_stop: async_std::channel::Receiver<()>,
    ) -> std::io::Result<()> {
        listen(event_stop).await
    }
}

pub async fn listen(event_stop: async_std::channel::Receiver<()>) -> std::io::Result<()> {
    let addr = "localhost:8080";
    let listener = net::TcpListener::bind(addr).await?;
    let mut new_connections = listener.incoming();

    loop {
        if let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            task::spawn(async {
                serve(socket).await;
            });
        }
        if let Ok(_) = event_stop.try_recv() {
            break;
        }
    }

    Ok(())
}

pub async fn serve(socket: net::TcpStream) {
    println!("Connection from {}", socket.peer_addr().unwrap());
}
