// Input: n = 1
// Output: "foobar"
// Explanation: There are two threads being fired asynchronously. One of them calls foo(), while the other calls bar().
// "foobar" is being output 1 time.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct FooBar {
    state: Mutex<bool>, // true for foo's turn, false for bar's turn
    cond: Condvar,
}

impl FooBar {
    fn new() -> Self {
        FooBar {
            state: Mutex::new(true), // Start with foo's turn
            cond: Condvar::new(),
        }
    }

    fn foo(&self) {
        let mut state = self.state.lock().unwrap();
        // Print "foo" and switch state to bar's turn.
        print!("foo");
        *state = false;
        // Notify the other thread that the state has changed.
        self.cond.notify_one();
    }

    fn bar(&self) {
        let mut state = self.state.lock().unwrap();
        // Wait until it's bar's turn.
        while *state {
            state = self.cond.wait(state).unwrap();
        }
        // Print "bar" and switch state to foo's turn.
        println!("bar");
        *state = true;
        // Notify the other thread that the state has changed.
        self.cond.notify_one();
    }
}

fn main() {
    let foobar = Arc::new(FooBar::new());
    let foobar_clone = Arc::clone(&foobar);

    let foo_thread = thread::spawn(move || {
        foobar.foo();
    });

    let bar_thread = thread::spawn(move || {
        foobar_clone.bar();
    });

    foo_thread.join().unwrap();
    bar_thread.join().unwrap();
}
