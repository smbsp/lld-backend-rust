// Method Overloading - compile-time polymorphism

// Generics (Static Polymorphism) - Generics allow for compile-time polymorphism, where the same function or 
// struct can operate on different types specified at compile time.

fn print_info<T: std::fmt::Display>(item: T) {
    println!("The value is: {}", item);
}

fn main() {
    print_info(42); // Integer
    print_info("Hello, world!"); // &str
}
