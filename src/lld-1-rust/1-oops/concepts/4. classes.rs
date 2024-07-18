// Rust does not have classes in the same way that object-oriented languages like 
// Java or C++ do. Instead, Rust uses structs and enums to create custom data types, and it 
// uses traits to define shared behavior.

// 1. The Person struct and its methods are defined in the program's code memory.
// 2. The person variable is stored on the stack, containing the stack-allocated age field and a pointer to 
// the heap-allocated name field.
// 3. The string data for the name field is stored on the heap.

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

fn main() {
    let person = Person::new("Alice".to_string(), 30);
    person.greet();
}
