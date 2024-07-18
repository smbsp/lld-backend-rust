// Deref coercion is a feature in Rust that allows for automatic conversion of references to types that implement the Deref trait 
// into references to other types.

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref coercion allows us to use *y to get the value inside the box
}
