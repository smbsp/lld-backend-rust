// Composition - Instead of inheritance, Rust encourages the use of composition to achieve code reuse 
// and polymorphism. This involves creating structs that contain other structs or traits as fields.

// Define a trait `Engine` with a method `start`
trait Engine {
    fn start(&self);
}

// Implement the `Engine` trait for a struct `GasolineEngine`
struct GasolineEngine;

impl Engine for GasolineEngine {
    fn start(&self) {
        println!("Gasoline engine starts!");
    }
}

// Define a struct `Car` that contains an engine
struct Car<E: Engine> {
    engine: E,
}

// Implement a method `start` for `Car` that starts its engine
impl<E: Engine> Car<E> {
    fn start(&self) {
        self.engine.start();
    }
}

fn main() {
    let gasoline_car = Car {
        engine: GasolineEngine,
    };

    gasoline_car.start();
}

// A simpler example for understanding

// Define a struct `Engine` with some properties
struct Engine {
    horsepower: u32,
    fuel_type: String,
}

// Define a struct `Transmission` with some properties
struct Transmission {
    gear_count: u32,
    type_: String,
}

// Define a struct `Car` that is composed of `Engine` and `Transmission`
struct Car {
    make: String,
    model: String,
    engine: Engine,
    transmission: Transmission,
}

impl Car {
    // Constructor for `Car` that takes ownership of an `Engine` and a `Transmission`
    fn new(make: String, model: String, engine: Engine, transmission: Transmission) -> Self {
        Car {
            make,
            model,
            engine,
            transmission,
        }
    }

    // Method to display information about the car
    fn display_info(&self) {
        println!("Make: {}, Model: {}", self.make, self.model);
        println!(
            "Engine: {} HP, Fuel: {}",
            self.engine.horsepower, self.engine.fuel_type
        );
        println!(
            "Transmission: {} gears, Type: {}",
            self.transmission.gear_count, self.transmission.type_
        );
    }
}

fn main() {
    let car_engine = Engine {
        horsepower: 300,
        fuel_type: "Petrol".to_string(),
    };

    let car_transmission = Transmission {
        gear_count: 6,
        type_: "Manual".to_string(),
    };

    let car = Car::new(
        "Ford".to_string(),
        "Mustang".to_string(),
        car_engine,
        car_transmission,
    );

    car.display_info();
}
