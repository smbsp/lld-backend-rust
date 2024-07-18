// You have an object printNumber. printNumber.accept(x) can be called with an integer parameter that prints it to the console.

// You are given an instance of the class ZeroEvenOdd that has three functions: zero, even, and odd. The same instance of ZeroEvenOdd will be passed to three different threads:

//     Thread A: calls zero() that should only output 0's.
//     Thread B: calls even() that should only output even numbers.
//     Thread C: calls odd() that should only output odd numbers.

// Modify the given class to output the series "010203040506..." where the length of the series must be 2n.

// Implement the ZeroEvenOdd class:

//     ZeroEvenOdd(int n) Initializes the object with the number n that represents the numbers that should be printed.
//     void zero(printNumber) Calls printNumber to output one zero.
//     void even(printNumber) Calls printNumber to output one even number.
//     void odd(printNumber) Calls printNumber to output one odd number.

// Example 1:

// Input: n = 2
// Output: "0102"
// Explanation: There are three threads being fired asynchronously.
// One of them calls zero(), the other calls even(), and the last one calls odd().
// "0102" is the correct output.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct ZeroEvenOdd {
    n: i32,
    current: Mutex<(i32, bool)>, // (current number, is_zero_turn)
    cond: Condvar,
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {
        ZeroEvenOdd {
            n,
            current: Mutex::new((0, true)), // Start with zero's turn
            cond: Condvar::new(),
        }
    }

    fn zero(&self, print_number: impl Fn(i32) + Send + 'static) {
        for _ in 0..self.n {
            let mut current = self.current.lock().unwrap();
            while !current.1 {
                current = self.cond.wait(current).unwrap();
            }
            print_number(0);
            current.1 = false; // Switch to number's turn
            self.cond.notify_all();
        }
    }

    fn even(&self, print_number: impl Fn(i32) + Send + 'static) {
        for i in 1..=self.n {
            if i % 2 == 0 {
                let mut current = self.current.lock().unwrap();
                while current.1 || current.0 != i - 1 {
                    current = self.cond.wait(current).unwrap();
                }
                print_number(i);
                current.0 = i; // Update the current number
                current.1 = true; // Switch to zero's turn
                self.cond.notify_all();
            }
        }
    }

    fn odd(&self, print_number: impl Fn(i32) + Send + 'static) {
        for i in 1..=self.n {
            if i % 2 != 0 {
                let mut current = self.current.lock().unwrap();
                while current.1 || current.0 != i - 1 {
                    current = self.cond.wait(current).unwrap();
                }
                print_number(i);
                current.0 = i; // Update the current number
                current.1 = true; // Switch to zero's turn
                self.cond.notify_all();
            }
        }
    }
}

fn main() {
    let zero_even_odd = Arc::new(ZeroEvenOdd::new(4));
    let print_number = |x: i32| {
        println!("{}", x);
        // io::stdout().flush().unwrap(); // Flush the output to ensure it's printed immediately.
    };

    let zero_even_odd_clone = Arc::clone(&zero_even_odd);
    let zero_thread = thread::spawn(move || {
        zero_even_odd_clone.zero(print_number);
    });

    let zero_even_odd_clone = Arc::clone(&zero_even_odd);
    let even_thread = thread::spawn(move || {
        zero_even_odd_clone.even(print_number);
    });

    let zero_even_odd_clone = Arc::clone(&zero_even_odd);
    let odd_thread = thread::spawn(move || {
        zero_even_odd_clone.odd(print_number);
    });

    zero_thread.join().unwrap();
    even_thread.join().unwrap();
    odd_thread.join().unwrap();
}
