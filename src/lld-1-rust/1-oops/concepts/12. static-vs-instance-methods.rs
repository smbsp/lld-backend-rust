// Static vs Instance Methods

// In Rust, static methods are associated functions that are called on the type itself 
// rather than on an instance of the type. They are similar to class methods in other languages. 
// Static methods are defined within an impl block for the type, using the fn keyword without 
// taking self as the first parameter.

// Instance methods in Rust are functions defined within an impl block for a type that 
// take self, &self, or &mut self as their first parameter. These methods are called on 
// instances of the type, and they can access and modify the instance's data.

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Constructor: a static method that creates a new instance of `Point`
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Instance method: called on an instance of `Point`
    // Takes an immutable reference to `self`
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    // Instance method: called on an instance of `Point`
    // Takes a mutable reference to `self`
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

fn main() {
    let mut point = Point::new(3, 4);

    // Calling an instance method on `point`
    println!("Distance from origin: {}", point.distance_from_origin());

    // Calling another instance method to modify `point`
    point.move_by(1, 1);
    println!("New position: ({}, {})", point.x, point.y);
}
