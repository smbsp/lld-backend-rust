// In Rust, there is no direct concept of "constructor overloading" as seen in some other 
// object-oriented languages. Rust does not have a built-in notion of constructors in the same 
// way that languages like Java or C++ do. Instead, Rust uses associated functions 
// (commonly called "associated constructors" or just "constructors") to create instances of a struct.

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Constructor for creating a point with specific x and y coordinates
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // Constructor for creating a point at the origin (0, 0)
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    // Constructor for creating a point with the same x and y coordinates
    fn uniform(value: i32) -> Point {
        Point { x: value, y: value }
    }
}

fn main() {
    let p1 = Point::new(2, 3);
    let p2 = Point::origin();
    let p3 = Point::uniform(5);

    println!("p1: ({}, {})", p1.x, p1.y);
    println!("p2: ({}, {})", p2.x, p2.y);
    println!("p3: ({}, {})", p3.x, p3.y);
}
