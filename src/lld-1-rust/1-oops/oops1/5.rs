// Create a class Point. It should have 2 data-members:
//     x:int
//     y:int

// Create a class Rectangle. It should have 3 data-members
//     topLeft:Point
//     height:int
//     width:int

// It should have 3 methods
//     getArea: It should return area of rectangle as an integer
//     getPerimeter: It should return perimeter of rectangle as an integer
//     getBottomRight: It should return a Point to represent the bottom right of the Rectangle.

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    top_left: Point,
    height: i32,
    width: i32,
}

impl Rectangle {
    fn get_area(&self) -> i32 {
        self.height * self.width
    }

    fn get_perimeter(&self) -> i32 {
        2 * (self.height + self.width)
    }

    fn get_bottom_right(&self) -> Point {
        Point {
            x: self.top_left.x + self.width,
            y: self.top_left.y - self.height,
        }
    }
}

fn main() {
    let top_left: Point = Point { x: 0, y: 10 };
    let rect: Rectangle = Rectangle {
        top_left,
        height: 5,
        width: 10,
    };

    println!("Area: {}", rect.get_area());
    println!("Perimeter: {}", rect.get_perimeter());
    
    let bottom_right: Point = rect.get_bottom_right();
    println!("Bottom Right: ({}, {})", bottom_right.x, bottom_right.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area() {
        let top_left: Point = Point {
            x: 0,
            y: 10,
        };

        let rectangle = Rectangle {
            top_left,
            height: 5,
            width: 10,
        };

        let output = rectangle.get_area();
        assert_eq!(output, 50);
    }

    #[test]
    fn test_get_perimeter() {
        let top_left: Point = Point {
            x: 0,
            y: 10,
        };

        let rectangle = Rectangle {
            top_left,
            height: 5,
            width: 10,
        };

        let output = rectangle.get_perimeter();
        assert_eq!(output, 30);
    }

    #[test]
    fn test_get_bottom_right() {
        let top_left: Point = Point {
            x: 0,
            y: 10,
        };

        let rectangle = Rectangle {
            top_left,
            height: 5,
            width: 10,
        };
        
        let bottom_right: Point = rectangle.get_bottom_right();
        assert_eq!(bottom_right.x, 10);
        assert_eq!(bottom_right.y, 5);
    }
}