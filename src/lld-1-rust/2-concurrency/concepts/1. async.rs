// Asynchronous Programming in Rust is primarily achieved using async and await syntax, along with futures. 
// It allows the program to perform other tasks while waiting for IO-bound or long-running operations to 
// complete, without blocking the execution thread.

// Runtime and Language Integration - Tokio is a runtime for Rust that provides asynchronous I/O and task 
// scheduling. It's an external library that you include in your Rust projects.

// Task Scheduling - Tokio uses a multi-threaded scheduler by default, which can execute tasks across multiple 
// OS threads in a thread pool. This allows for true parallelism on multi-core systems.

// Futures in Rust are more akin to JavaScript's promises, but their execution is managed by the Tokio runtime's 
// task scheduler. In Tokio, asynchronous tasks are scheduled by the runtime, and their execution order 
// can be influenced by factors like task priorities and which thread in the pool is available to execute them.

use std::time::Duration;
use tokio; // Tokio is a popular runtime for writing async Rust applications.

#[tokio::main]
async fn main() {
    let task_one = async_operation();
    let task_two = async_operation();

    // Await both tasks concurrently, without blocking the thread.
    let (result_one, result_two) = tokio::join!(task_one, task_two);
    println!("Results: {}, {}", result_one, result_two);
}

async fn async_operation() -> i32 {
    tokio::time::sleep(Duration::from_secs(1)).await;
    42
}

// use tokio::time::{sleep, Duration};

// #[tokio::main]
// async fn main() {
//     let task1 = async {
//         println!("Task 1 started");
//         sleep(Duration::from_secs(2)).await;
//         println!("Task 1 completed");
//     };

//     let task2 = async {
//         println!("Task 2 started");
//         sleep(Duration::from_secs(1)).await;
//         println!("Task 2 completed");
//     };

//     println!("Running tasks concurrently");
//     tokio::join!(task1, task2);
//     println!("All tasks completed");
// }