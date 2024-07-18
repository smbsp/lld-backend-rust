// Write code to achieve the following

//     A class Client with a main method.
//     Create a class ArrayCreator which takes as input a number (n)
//     ArrayCreator should create an ArrayList which should contain numbers from 1 to n
//     ArrayCreator should implement appropriate Callable interface and return the arraylist discussed above to calling thread
//     Client class should invoke ArrayCreator over a new thread and get the arraylist from ArrayCreator class and print it.

use std::sync::{Arc, Mutex}; // Import Arc and Mutex from the standard library for thread-safe reference counting and mutual exclusion.
use std::thread; // Import the thread module for creating new threads.

// Define a struct ArrayCreator that holds a single field `n` of type usize.
struct ArrayCreator {
    n: usize,
}

// Implement methods for the ArrayCreator struct.
impl ArrayCreator {
    // Constructor method to create a new ArrayCreator instance with a given value `n`.
    fn new(n: usize) -> Self {
        ArrayCreator { n }
    }

    // Method to create an array (Vec<usize>) containing numbers from 1 to `n`.
    fn create_array(&self) -> Vec<usize> {
        (1..=self.n).collect() // Use a range and collect it into a vector.
    }
}

fn main() {
    let array_creator = ArrayCreator::new(10); // Create a new ArrayCreator instance with `n` set to 10.
    let result = Arc::new(Mutex::new(Vec::new())); // Create a new Arc containing a Mutex, which in turn contains an empty vector.

    let result_clone = Arc::clone(&result); // Clone the Arc to share ownership with the new thread.
    let handle = thread::spawn(move || { // Spawn a new thread.
        let array = array_creator.create_array(); // Create the array in the new thread.
        let mut result = result_clone.lock().unwrap(); // Lock the mutex to get mutable access to the vector.
        *result = array; // Update the vector with the newly created array.
    });

    handle.join().unwrap(); // Wait for the new thread to finish execution.

    let result = result.lock().unwrap(); // Lock the mutex again to get access to the updated vector.
    println!("{:?}", *result); // Print the contents of the vector.
}

