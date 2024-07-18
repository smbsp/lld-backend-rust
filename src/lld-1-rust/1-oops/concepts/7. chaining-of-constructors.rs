struct Person {
    name: String,
    age: u32,
    height: f32,
}

impl Person {
    // Primary constructor
    fn new(name: String, age: u32, height: f32) -> Self {
        Self { name, age, height }
    }

    // Secondary constructor that chains to the primary constructor
    fn new_with_default_height(name: String, age: u32) -> Self {
        Self::new(name, age, 1.75) // Assuming default height is 1.75 meters
    }
}

fn main() {
    let person1 = Person::new("Alice".to_string(), 30, 1.68);
    let person2 = Person::new_with_default_height("Bob".to_string(), 25);

    println!("{} is {} years old and {} meters tall.", person1.name, person1.age, person1.height);
    println!("{} is {} years old and {} meters tall.", person2.name, person2.age, person2.height);
}
