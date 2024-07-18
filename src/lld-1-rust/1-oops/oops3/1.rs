// Create a set of class with following requirements:

// a. Create a class Point

//     i. It should have two data members
//         x:int
//         y: int
//     It should have a display method that prints in following format - “[<<x>>, <<y>>]”
//     x and y should be protected and display should be public


// b. Create another class ThreedPoint which extends the Point class

//     It should have the following data members: z:int
//     It should have a display method to override the parent’s display method which prints in following 
//     format - “[<<x>>, <<y>>, <<z>>]”
//     z should be private and display should be public.

// Define the Point class
struct Point {
    protected_x: i32,
    protected_y: i32,
}

impl Point {
    // Constructor for Point
    fn new(x: i32, y: i32) -> Self {
        Point { protected_x: x, protected_y: y }
    }

    // Public display method for Point
    fn display(&self) {
        println!("[{}, {}]", self.protected_x, self.protected_y);
    }
}

// Define the ThreeDPoint class, extending Point
struct ThreeDPoint {
    base: Point,
    private_z: i32,
}

impl ThreeDPoint {
    // Constructor for ThreeDPoint
    fn new(x: i32, y: i32, z: i32) -> Self {
        ThreeDPoint {
            base: Point::new(x, y),
            private_z: z,
        }
    }

    // Public display method for ThreeDPoint, overriding Point's display
    fn display(&self) {
        println!("[{}, {}, {}]", self.base.protected_x, self.base.protected_y, self.private_z);
    }
}

fn main() {
    let point = Point::new(1, 2);
    point.display();

    let three_d_point = ThreeDPoint::new(3, 4, 5);
    three_d_point.display();
}
