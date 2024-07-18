// Method Overriding - runtime polymorphism

// Trait Objects (Dynamic Polymorphism) - Trait objects allow for runtime polymorphism, where the 
// exact type of an object can vary at runtime, as long as it implements a specific trait.

trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn make_animal_speak(animal: &dyn Animal) {
    animal.speak();
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    make_animal_speak(&dog);
    make_animal_speak(&cat);
}


// There are other patterns and techniques that can be used to achieve similar dynamic behavior.

// 1. Enums with Variants: You can use enums with variants that hold different types, 
// each implementing a common trait. This approach allows you to match on the enum and call 
// methods based on the variant, but it requires knowing all possible types at compile time.

trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

enum Pet {
    Dog(Dog),
    Cat(Cat),
}

impl Animal for Pet {
    fn speak(&self) {
        match self {
            Pet::Dog(dog) => dog.speak(),
            Pet::Cat(cat) => cat.speak(),
        }
    }
}

fn main() {
    let pet: Pet = Pet::Dog(Dog);
    pet.speak();
}

// 2. Function Pointers: For simpler cases, you can use function pointers to achieve a form of 
// dynamic behavior. This is more limited than trait objects and is typically used for callbacks 
// or simple polymorphic behavior.

fn greet() {
    println!("Hello!");
}

fn farewell() {
    println!("Goodbye!");
}

fn call_function(f: fn()) {
    f();
}

fn main() {
    let f: fn() = greet;
    call_function(f);
    let f: fn() = farewell;
    call_function(f);
}

// 3. Closures: Closures in Rust can capture their environment and can be used to achieve dynamic 
// behavior similar to function pointers, but with more flexibility. They can be stored 
// in variables and passed around as arguments.

fn main() {
    let greet = || println!("Hello!");
    let farewell = || println!("Goodbye!");

    let mut f: Box<dyn Fn()> = Box::new(greet);
    f();

    f = Box::new(farewell);
    f();
}
