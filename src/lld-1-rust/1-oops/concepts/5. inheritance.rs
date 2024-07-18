// In Rust, there is no direct support for traditional inheritance as seen in object-oriented languages 
// like Java or C++. Instead, Rust provides alternative mechanisms for achieving similar behavior, 
// such as trait inheritance and composition.

// Define a base trait `Animal` with a method `make_sound`
trait Animal {
    fn make_sound(&self);
}

// Define a trait `Bird` that inherits from `Animal`
trait Bird: Animal {
    fn fly(&self);
}

// Implement the `Animal` and `Bird` traits for a struct `Sparrow`
struct Sparrow;

impl Animal for Sparrow {
    fn make_sound(&self) {
        println!("Chirp!");
    }
}

impl Bird for Sparrow {
    fn fly(&self) {
        println!("The sparrow is flying.");
    }
}

fn main() {
    let sparrow = Sparrow;
    sparrow.make_sound();
    sparrow.fly();
}
