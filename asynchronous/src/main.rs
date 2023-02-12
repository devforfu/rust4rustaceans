use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::executor;

struct Data(Vec<u8>);

impl Future for Data {
    type Output = Vec<u8>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.0.clone())
    }
}

fn main() {
    let future_data = async {
        Data(vec![1, 2, 3]).await
    };

    let values = executor::block_on(future_data);

    println!("Awaited values: {values:?}");
}
