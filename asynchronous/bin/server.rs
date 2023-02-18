use async_lib::net::Listener;
use futures::executor::block_on;

fn main() -> std::io::Result<()> {
    block_on(async { Listener::new("localhost:8080").listen_forever().await })
}
