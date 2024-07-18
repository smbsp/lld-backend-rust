// Write code to achieve the following

//     A class Client with a main method.
//     Client class shall take two numbers as input from the user.
//     Client class should create a new thread and invoke code in a class called Adder.
//     Client class shall pass two numbers (taken as input from the user) to the Adder class.
//     The Adder class should print the sum of two numbers passed to it.

use std::io;
use std::thread;

// Define the Adder struct
struct Adder;

// Implement a method for Adder that takes two numbers and prints their sum
impl Adder {
    fn add_and_print(a: i32, b: i32) {
        println!("Sum of {} and {} is {}", a, b, a + b);
    }
}

// Define the Client struct
struct Client;

// Implement the main method for Client
impl Client {
    fn main() {
        // Take two numbers as input from the user
        let mut input = String::new();
        println!("Enter the first number:");
        io::stdin().read_line(&mut input).unwrap();
        let num1: i32 = input.trim().parse().unwrap();

        input.clear();
        println!("Enter the second number:");
        io::stdin().read_line(&mut input).unwrap();
        let num2: i32 = input.trim().parse().unwrap();

        // Create a new thread to invoke Adder's code with the two numbers
        let adder_thread = thread::spawn(move || {
            Adder::add_and_print(num1, num2);
        });

        // Wait for the thread to complete
        adder_thread.join().unwrap();
    }
}

fn main() {
    Client::main();
}
