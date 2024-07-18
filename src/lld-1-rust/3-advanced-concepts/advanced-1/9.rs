// Car with lower speed is considered smaller.

use std::cmp::Ordering;

#[derive(Debug)]
struct Car {
    speed: i32,
    #[allow(dead_code)]
    power: i32,
}

impl Car {
    fn new(speed: i32, power: i32) -> Self {
        Car { speed, power }
    }
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.speed == other.speed
    }
}

impl Eq for Car {}

impl PartialOrd for Car {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Car {
    fn cmp(&self, other: &Self) -> Ordering {
        self.speed.cmp(&other.speed)
    }
}

fn main() {
    let mut cars = vec![
        Car::new(150, 200),
        Car::new(120, 250),
        Car::new(180, 180),
    ];

    cars.sort();

    for car in cars {
        println!("{:?}", car);
    }
}
