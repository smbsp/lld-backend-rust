// Abstraction in Rust is a way to hide the complex implementation details of a piece of code 
// and expose only the essential features that are necessary for using it. 
// This is achieved through the use of traits and generics, which allow for flexible 
// and reusable code without exposing the underlying details.

// Define a trait `Shape` that abstracts the behavior of shapes
trait Shape {
    fn area(&self) -> f64;
}

// Implement the `Shape` trait for a `Rectangle` struct
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Implement the `Shape` trait for a `Circle` struct
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// A function that takes a reference to a `Shape` and prints its area
fn print_area<T: Shape>(shape: &T) {
    println!("The area of the shape is {}", shape.area());
}

fn main() {
    let rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let circle = Circle {
        radius: 4.0,
    };

    // Use the `print_area` function with different shapes
    print_area(&rectangle);
    print_area(&circle);
}
