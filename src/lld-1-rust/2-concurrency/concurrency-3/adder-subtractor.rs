// Problem - Both threads to access and modify the counter variable concurrently without any synchronization
// Solution
// We use a Mutex to protect the shared counter variable. The lock method is used to acquire the mutex, and the lock is automatically 
// released when the MutexGuard goes out of scope (at the end of the block). We create two threads: the adder thread increments the 
// counter 1,000,000 times, and the subtractor thread decrements the counter 1,000,000 times. We use an Arc (atomic reference counter) 
// to share the Mutex-protected counter between threads safely.
// After both threads have completed their execution, we print the final value of the counter, which should be 0, as each increment by 
// the adder thread is exactly offset by a decrement by the subtractor thread.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Adder thread
    let counter_clone = Arc::clone(&counter);
    let adder = thread::spawn(move || {
        for _ in 0..1_000_000 {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }
    });
    handles.push(adder);

    // Subtractor thread
    let counter_clone = Arc::clone(&counter);
    let subtractor = thread::spawn(move || {
        for _ in 0..1_000_000 {
            let mut num = counter_clone.lock().unwrap();
            *num -= 1;
        }
    });
    handles.push(subtractor);

    // Wait for both threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}
