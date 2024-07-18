use std::io;
use std::thread;

struct TableCreator;

impl TableCreator {
    fn print_table(x: i32) {
        let mut threads = vec![];
        for i in 1..=10 {
            let thread = thread::spawn(move || {
                let result = x * i;
                let thread_id = thread::current().id();
                println!("Thread {:?}: {} times {} is {}", thread_id, x, i, result);
            });
            threads.push(thread);
        }

        // Wait for all multiplication threads to complete
        for thread in threads {
            thread.join().unwrap();
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

        for x in 1..=n {
            TableCreator::print_table(x);
        }
    }
}

fn main() {
    Client::main();
}
