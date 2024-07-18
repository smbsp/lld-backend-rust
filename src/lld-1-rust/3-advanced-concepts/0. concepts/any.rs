// In Rust, most types automatically implement the Any trait, so you can generally assume that any type you're 
// working with implements Any. This includes all primitive types, structs, enums, and even references to types that implement Any.

// &str does not implement Any due to its non-'static lifetime

use std::any::Any;

fn print_type_name<T: Any>(_value: &T) {
    let type_name = std::any::type_name::<T>();
    println!("Type name: {}", type_name);
}

fn main() {
    let number = 10;
    let text = "Hello";

    print_type_name(&number);
    print_type_name(&text);
}
