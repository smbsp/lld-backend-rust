// Rust supports operator overloading. You can overload various operators for your own types by 
// implementing specific traits defined in the std::ops module. Each operator is associated with 
// a corresponding trait, and by implementing that trait for your type, you can define the 
// behavior of the operator for that type.

use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2; // Uses the overloaded + operator
    println!("p3: ({}, {})", p3.x, p3.y);
}
