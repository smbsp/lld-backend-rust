// We intend to use the points in a HashSet

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

// Implement the PartialEq trait for Point to enable comparison
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Implement the Eq trait for Point since it has no additional requirements beyond PartialEq
impl Eq for Point {}

// Implement the Hash trait for Point to enable its use in a HashSet
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn main() {
    let mut points = HashSet::new();
    points.insert(Point::new(1, 2));
    points.insert(Point::new(3, 4));
    points.insert(Point::new(1, 2)); // This will not be added as it's a duplicate

    for point in &points {
        println!("Point: ({}, {})", point.x, point.y);
    }
}
