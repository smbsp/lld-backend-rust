// Input: nums = [1,2,3]
// Output: "firstsecondthird"
// Explanation: There are three threads being fired asynchronously. The input [1,2,3] means thread A calls 
// first(), thread B calls second(), and thread C calls third(). "firstsecondthird" is the correct output.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct Foo {
    state: Mutex<i32>,
    cond: Condvar,
}

impl Foo {
    fn new() -> Self {
        Foo {
            state: Mutex::new(0),
            cond: Condvar::new(),
        }
    }

    fn first(&self) {
        let mut state = self.state.lock().unwrap();
        // Print "first".
        print!("first");
        *state = 1;
        // Notify other threads that the state has changed.
        self.cond.notify_all();
    }

    fn second(&self) {
        let mut state = self.state.lock().unwrap();
        // Wait until the state is 1.
        while *state != 1 {
            state = self.cond.wait(state).unwrap();
        }
        // Print "second".
        print!("second");
        *state = 2;
        // Notify other threads that the state has changed.
        self.cond.notify_all();
    }

    fn third(&self) {
        let mut state = self.state.lock().unwrap();
        // Wait until the state is 2.
        while *state != 2 {
            state = self.cond.wait(state).unwrap();
        }
        // Print "third".
        println!("third");
    }
}

fn main() {
    let foo = Arc::new(Foo::new());

    let foo_clone1 = Arc::clone(&foo);
    let foo_clone2 = Arc::clone(&foo);
    let foo_clone3 = Arc::clone(&foo);

    let thread_a = thread::spawn(move || {
        foo_clone1.first();
    });

    let thread_b = thread::spawn(move || {
        foo_clone2.second();
    });

    let thread_c = thread::spawn(move || {
        foo_clone3.third();
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}
