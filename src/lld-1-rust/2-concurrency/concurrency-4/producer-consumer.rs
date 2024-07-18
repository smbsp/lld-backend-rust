use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::collections::VecDeque;

const BUFFER_SIZE: usize = 5;

struct Semaphore {
    mutex: Mutex<usize>,
    condvar: Condvar,
}

impl Semaphore {
    fn new(count: usize) -> Self {
        Semaphore {
            mutex: Mutex::new(count),
            condvar: Condvar::new(),
        }
    }

    fn acquire(&self) {
        let mut count = self.mutex.lock().unwrap();
        while *count == 0 {
            count = self.condvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    fn release(&self) {
        let mut count = self.mutex.lock().unwrap();
        *count += 1;
        self.condvar.notify_one();
    }
}

fn main() {
    let buffer = Arc::new(Mutex::new(VecDeque::new()));
    let empty_slots = Arc::new(Semaphore::new(BUFFER_SIZE));
    let full_slots = Arc::new(Semaphore::new(0));

    // Producer thread
    let empty_slots_clone = Arc::clone(&empty_slots);
    let full_slots_clone = Arc::clone(&full_slots);
    let buffer_clone = Arc::clone(&buffer);
    let producer = thread::spawn(move || {
        for i in 1..=10 {
            empty_slots_clone.acquire();
            let mut buffer = buffer_clone.lock().unwrap();
            buffer.push_back(i);
            println!("Produced: {}", i);
            full_slots_clone.release();
        }
    });

    // Consumer thread
    let empty_slots_clone = Arc::clone(&empty_slots);
    let full_slots_clone = Arc::clone(&full_slots);
    let buffer_clone = Arc::clone(&buffer);
    let consumer = thread::spawn(move || {
        for _ in 1..=10 {
            full_slots_clone.acquire();
            let mut buffer = buffer_clone.lock().unwrap();
            if let Some(data) = buffer.pop_front() {
                println!("Consumed: {}", data);
            }
            empty_slots_clone.release();
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
