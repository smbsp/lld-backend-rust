// Write code to achieve the following

//     A class Client with a main method.
//     Client class shall take a number n as input.
//     A class called TableCreator which prints the multiplication table from 1 to 10 for a given number x
//     x times 1 is x

//     ..

//     x times 10 is 10x
//     Client should create a thread for every number between 1 to n, n included and
//     Pass it to TableCreator to print a multiplication table for that number.

// Print format = 2 times 6 is 12

use std::io;
use std::thread;

struct TableCreator;

impl TableCreator {
    fn print_table(x: i32) {
        let binding = thread::current();
        let thread_name = binding.name().unwrap_or("Unnamed thread");
        for i in 1..=10 {
            println!("{}: {} times {} is {}", thread_name, x, i, x * i);
        }
    }
}

struct Client;

impl Client {
    fn main() {
        println!("Enter a number n:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        let mut threads = vec![];
        for x in 1..=n {
            // Use thread::Builder to set a custom name for each thread
            let thread = thread::Builder::new()
                .name(format!("Thread-{}", x)) // Set the thread name
                .spawn(move || {
                    TableCreator::print_table(x);
                })
                .expect("Failed to spawn thread");

            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }
}

fn main() {
    Client::main();
}

