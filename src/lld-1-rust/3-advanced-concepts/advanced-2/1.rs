// Reflection - In Rust, the visibility of struct fields is determined by their access modifiers. By default, 
// struct fields are private, meaning they can only be accessed within the module where they are defined. 
// However, this does not affect the ability of serialization libraries like serde to access and serialize these fields.

// In contrast, Java's reflection allows for examining and manipulating the structure and behavior of objects and classes at runtime, 
// including accessing field types, method signatures, and more. This level of runtime introspection is not natively available in Rust 
// due to its focus on compile-time safety and efficiency.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::type_name;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn get_point_fields() -> Vec<String> {
    let point = Point { x: 0, y: 0 };
    let serialized = serde_json::to_value(&point).unwrap();
    match serialized {
        Value::Object(map) => map.keys().cloned().collect(),
        _ => vec![],
    }
}

fn get_point_field_types() -> Vec<String> {
    vec![type_name::<i32>().to_string(), type_name::<i32>().to_string()]
}

fn main() {
    println!("Assuming the Point class has two fields: x and y, both of type i32.");
    println!();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice: i32 = input.trim().parse().unwrap();

    if choice == 1 {
        println!("If the user inputs 1, the output will be:");
        let fields = get_point_fields();
        for field in fields {
            println!("{}", field);
        }
    } else {
        println!("If the user inputs any number other than 1, the output will be:");
        let field_types = get_point_field_types();
        for field_type in field_types {
            println!("{}", field_type);
        }
    }
}
