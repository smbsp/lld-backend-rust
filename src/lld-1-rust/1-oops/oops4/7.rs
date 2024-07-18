// Write a class Car with following requirements

//     It should have 2 data-members
//         Price: int
//         Speed: int
//     We should be able to sort a Collection or Array of Cars on price.
//     Implement required interface for that

#[derive(Debug, PartialEq, Eq)]
struct Car {
    price: i32,
    speed: i32,
}

impl PartialOrd for Car {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Car {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

fn main() {
    let mut cars = vec![
        Car { price: 20000, speed: 150 },
        Car { price: 15000, speed: 120 },
        Car { price: 30000, speed: 200 },
    ];

    println!("Cars before sorting:");
    for car in &cars {
        println!("Price: {}, Speed: {}", car.price, car.speed);
    }

    // Sort the array of cars based on price
    cars.sort();

    // Sort cars by speed in ascending order
    // cars.sort_by(|a, b| a.price.cmp(&b.price));

    // Sort cars by price in ascending order
    // cars.sort_by_key(|car| car.price);

    println!("\nCars after sorting by price:");
    for car in &cars {
        println!("Price: {}, Speed: {}", car.price, car.speed);
    }
}
