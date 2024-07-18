// Concurrency in Rust involves managing multiple tasks at the same time but not necessarily executing 
// them simultaneously. It's about structure rather than execution, where the focus is on dealing with 
// many tasks at once.

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for _ in 0..5 {
            println!("Concurrently printing from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for _ in 0..5 {
        println!("Concurrently printing from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// Memory Allocation
// When the program starts, the operating system allocates memory for the process, including space for 
// the main thread's stack, heap, and other segments like the code and data segments.
// The thread::spawn function creates a new thread, which involves allocating memory for the new thread's stack. 
// The size of the stack is determined by the operating system or the runtime environment.
// The closure passed to thread::spawn captures its environment by value, which might involve copying data 
// from the main thread's stack or heap to the new thread's stack.

// Thread Creation
// The thread::spawn function requests the creation of a new thread from the operating system or the runtime's 
// thread scheduler. The operating system or runtime allocates necessary resources for the new thread, such as a 
// thread control block (TCB) and a unique thread identifier. The new thread starts executing the closure 
// provided to thread::spawn. In this case, it's a loop that prints a message and sleeps for a short duration.

// Processing
// Both the main thread and the spawned thread run concurrently. The exact timing of their execution depends on 
// the operating system's scheduling policy and the number of available CPU cores. The thread::sleep function 
// suspends the current thread for the specified duration, allowing other threads to run during that time.
// The main thread and the spawned thread might be running on different CPU cores if the system has multiple cores 
// and the operating system's scheduler decides to distribute the threads across cores. This can lead to true 
// parallelism.

// Synchronization
// The handle.join().unwrap() call in the main thread blocks until the spawned thread completes its execution. 
// This ensures that the main thread waits for the spawned thread to finish before exiting the program.
// The join method also synchronizes the memory between the main thread and the spawned thread, ensuring that 
// any writes made by the spawned thread are visible to the main thread after join returns.

// Architecture
// The underlying architecture of the system, including the CPU, memory, and operating system, plays a crucial 
// role in how threads are managed and executed. Modern CPUs have features like multiple cores and hyper-threading 
// that enable true parallel execution of threads. The operating system's kernel is responsible for managing 
// threads, scheduling their execution, and handling context switches between threads.