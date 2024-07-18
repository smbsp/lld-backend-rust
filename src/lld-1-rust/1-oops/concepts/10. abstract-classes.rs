// In Rust, there are no abstract classes in the same way as in object-oriented languages like 
// Java or C++. However, you can achieve similar behavior using traits with default method 
// implementations and trait objects.

trait Drawable {
    // A method with a default implementation
    fn draw(&self) {
        println!("Drawing a shape");
    }

    // An "abstract" method without a default implementation
    fn outline(&self);
}

// Implementing the trait for a struct `Circle`
struct Circle {
    radius: f32,
}

impl Drawable for Circle {
    // Providing an implementation for the "abstract" method
    fn outline(&self) {
        println!("Outlining a circle with radius {}", self.radius);
    }

    // Optionally overriding the default implementation
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

// Implementing the trait for a struct `Square`
struct Square {
    side: f32,
}

impl Drawable for Square {
    // Providing an implementation for the "abstract" method
    fn outline(&self) {
        println!("Outlining a square with side {}", self.side);
    }
    // No need to provide an implementation for `draw` unless we want to override the default behavior
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 10.0 };

    let shapes: Vec<&dyn Drawable> = vec![&circle, &square];

    for shape in shapes {
        shape.draw();
        shape.outline();
    }
}
