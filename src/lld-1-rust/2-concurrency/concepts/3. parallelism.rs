// Parallelism involves executing multiple operations at the same time. In Rust, this can be achieved 
// using threads or libraries like Rayon, which abstracts over threads and provides a data-parallelism model.

use rayon::prelude::*;

fn main() {
    let nums: Vec<_> = (1..=100).collect();
    let squares: Vec<_> = nums.par_iter().map(|&i| i * i).collect();

    println!("Squares: {:?}", squares);
}

// Achieving true parallelism with Tokio requires using its multi-threaded runtime, as the default runtime is single-threaded

// To check if the tasks are running on different cores, you can observe the thread IDs printed by each task. If the IDs are different, 
// it indicates that the tasks are running on different threads. However, whether these threads are running on different cores depends
// on the operating system's scheduling and the number of available cores on your machine.

use tokio::task;
use tokio::runtime::Runtime;

fn main() {
    // Create a multi-threaded Tokio runtime
    let runtime = Runtime::new().unwrap();

    // Spawn multiple asynchronous tasks
    runtime.block_on(async { // block the main thread until the asynchronous block completes
        let handle1 = task::spawn(async {
            // Simulate some work
            println!("Task 1 running on thread: {:?}", std::thread::current().id());
        });

        let handle2 = task::spawn(async {
            // Simulate some work
            println!("Task 2 running on thread: {:?}", std::thread::current().id());
        });

        // Wait for both tasks to complete
        handle1.await.unwrap();
        handle2.await.unwrap();
    });
}