// In Rust, the term "Future" refers to a trait that represents an asynchronous computation. A future is a 
// value that may not have been computed yet but will be available at some point in the future. The Future 
// trait is defined in the std::future module and is a central part of Rust's asynchronous programming model.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// A simple future that resolves to an i32 value after being polled twice.
struct MyFuture {
    count: u8,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count < 2 {
            self.count += 1;
            Poll::Pending
        } else {
            Poll::Ready(42)
        }
    }
}

fn main() {
    let mut future = MyFuture { count: 0 };
    let waker = futures::task::noop_waker();
    let mut cx = Context::from_waker(&waker);

    // Manually poll the future until it is ready.
    match Pin::new(&mut future).poll(&mut cx) {
        Poll::Pending => println!("Future is not ready yet"),
        Poll::Ready(value) => println!("Future is ready with value: {}", value),
    }

    match Pin::new(&mut future).poll(&mut cx) {
        Poll::Pending => println!("Future is not ready yet"),
        Poll::Ready(value) => println!("Future is ready with value: {}", value),
    }

    match Pin::new(&mut future).poll(&mut cx) {
        Poll::Pending => println!("Future is not ready yet"),
        Poll::Ready(value) => println!("Future is ready with value: {}", value),
    }
}
