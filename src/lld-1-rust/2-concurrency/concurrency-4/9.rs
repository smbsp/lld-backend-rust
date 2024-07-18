// There are two kinds of threads: oxygen and hydrogen. Your goal is to group these threads to form water molecules. There is a barrier 
// where each thread has to wait until a complete molecule can be formed. Hydrogen and oxygen threads will be given releaseHydrogen and 
// releaseOxygen methods respectively, which will allow them to pass the barrier. These threads should pass the barrier in groups 
// of three, and they must immediately bond with each other to form a water molecule. You must guarantee that all the threads 
// from one molecule bond before any other threads from the next molecule do. In other words:

//     If an oxygen thread arrives at the barrier when no hydrogen threads are present, it must wait for two hydrogen threads.
//     If a hydrogen thread arrives at the barrier when no other threads are present, it must wait for an oxygen thread and another hydrogen thread.

// We do not have to worry about matching the threads up explicitly; the threads do not necessarily know which other threads they are paired up with. 
// The key is that threads pass the barriers in complete sets; thus, if we examine the sequence of threads that bind and divide them into groups of 
// three, each group should contain one oxygen and two hydrogen threads. Write synchronization code for oxygen and hydrogen molecules that enforces these constraints.
// Example 1

// Input: water = "HOH"
// Output: "HHO"
// Explanation: "HOH" and "OHH" are also valid answers.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct H2O {
    counts: Mutex<(i32, i32)>, // (hydrogen_count, oxygen_count)
    cond: Condvar,
}

impl H2O {
    fn new() -> Self {
        H2O {
            counts: Mutex::new((0, 0)), // Initialize counts to (0, 0)
            cond: Condvar::new(),
        }
    }

    fn hydrogen(&self) {
        let mut counts = self.counts.lock().unwrap();
        counts.0 += 1; // Increment hydrogen count
        while counts.0 >= 2 && counts.1 >= 1 {
            self.cond.notify_all();
            counts.0 -= 2;
            counts.1 -= 1;
        }
        // Wait until it's time to form a water molecule
        while counts.0 > 2 || (counts.1 == 0 && counts.0 > 0) {
            counts = self.cond.wait(counts).unwrap();
        }
        println!("H");
        self.cond.notify_all();
    }

    fn oxygen(&self) {
        let mut counts = self.counts.lock().unwrap();
        counts.1 += 1; // Increment oxygen count
        while counts.0 >= 2 && counts.1 >= 1 {
            self.cond.notify_all();
            counts.0 -= 2;
            counts.1 -= 1;
        }
        // Wait until it's time to form a water molecule
        while counts.1 > 1 || (counts.0 < 2 && counts.1 > 0) {
            counts = self.cond.wait(counts).unwrap();
        }
        println!("O");
        self.cond.notify_all();
    }
}

fn main() {
    let h2o = Arc::new(H2O::new());
    let mut threads = vec![];

    for _ in 0..4 {
        let h2o_clone = Arc::clone(&h2o);
        threads.push(thread::spawn(move || {
            h2o_clone.hydrogen();
        }));
    }

    for _ in 0..2 {
        let h2o_clone = Arc::clone(&h2o);
        threads.push(thread::spawn(move || {
            h2o_clone.oxygen();
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}