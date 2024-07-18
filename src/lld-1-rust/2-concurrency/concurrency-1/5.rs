// Write code to achieve the following

//     A class Client with main method that prints: I am the main class
//     Client class should create a new thread and invoke code in a class called Adder.
//     The Adder class should print: I am the Adder class
//     Client class should create a new thread and invoke code in a class called Subtractor.
//     The Subtractor class should print: I am the Subtractor class

use std::thread;

// Define the Adder struct
struct Adder;

// Implement a method for Adder
impl Adder {
    fn print_message() {
        println!("I am the Adder class");
    }
}

// Define the Subtractor struct
struct Subtractor;

// Implement a method for Subtractor
impl Subtractor {
    fn print_message() {
        println!("I am the Subtractor class");
    }
}

// Define the Client struct
struct Client;

// Implement the main method for Client
impl Client {
    fn main() {
        println!("I am the main class");

        // Create a new thread to invoke Adder's code
        let adder_thread = thread::spawn(|| {
            Adder::print_message();
        });

        // Create a new thread to invoke Subtractor's code
        let subtractor_thread = thread::spawn(|| {
            Subtractor::print_message();
        });

        // Wait for both threads to complete
        adder_thread.join().unwrap();
        subtractor_thread.join().unwrap();
    }
}

fn main() {
    Client::main();
}
