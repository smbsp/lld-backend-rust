// Create a class with the following requirements

//     Class should be public and name is Counter
//     It should’ve a single private data member of int type named count
//     It should’ve a single public constructor which takes an integer parameter and sets the value of count
//     It should’ve following 3 methods
//         public void incValue(int offset)
//         This method should increment the value of count by offset. Also make this method synchronized

//         public void getValue()
//         This method should return the value of count. Also make this method synchronized

//         public void decValue(int offset)
//         This method should decrement the value of count by offset. Also make this method synchronized

use std::sync::Mutex;

pub struct Counter {
    count: Mutex<i32>,
}

impl Counter {
    pub fn new(initial_value: i32) -> Self {
        Counter {
            count: Mutex::new(initial_value),
        }
    }

    pub fn inc_value(&self, offset: i32) {
        let mut count = self.count.lock().unwrap();
        *count += offset;
    }

    pub fn get_value(&self) -> i32 {
        let count = self.count.lock().unwrap();
        *count
    }

    pub fn dec_value(&self, offset: i32) {
        let mut count = self.count.lock().unwrap();
        *count -= offset;
    }
}

fn main() {
    let counter = Counter::new(10);

    counter.inc_value(5);
    println!("Value after increment: {}", counter.get_value());

    counter.dec_value(3);
    println!("Value after decrement: {}", counter.get_value());
}
