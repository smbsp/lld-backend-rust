// Create the set of classes and interfaces to meet the following requirements

//     An interface I1 with following methods
//         fun1():void
//         fun(): void
//     An interface I2 with following methods
//         fun2():void
//         fun():void
//     An interface I extending both I1 and I2 with no methods
//     A class C implementing interface I


// Define the interface I1 as a trait
trait I1 {
    fn fun1(&self);
    fn fun(&self);
}

// Define the interface I2 as a trait
trait I2 {
    fn fun2(&self);
    fn fun(&self);
}

// Define the interface I as a trait that extends both I1 and I2
// In Rust, this is achieved by specifying trait bounds for I1 and I2
trait I: I1 + I2 {}

// Define the class C as a struct
struct C;

// Implement the I1 trait for C
impl I1 for C {
    fn fun1(&self) {
        println!("C::fun1");
    }

    fn fun(&self) {
        println!("C::fun from I1");
    }
}

// Implement the I2 trait for C
impl I2 for C {
    fn fun2(&self) {
        println!("C::fun2");
    }

    // Here, we provide a separate implementation for `fun` from I2
    // If you want the same implementation as I1, you can omit this
    fn fun(&self) {
        println!("C::fun from I2");
    }
}

// Since I has no additional methods, we can provide an empty implementation for C
impl I for C {}

fn main() {
    let c = C;
    c.fun1(); // Calls the method from I1
    c.fun2(); // Calls the method from I2
    // c.fun();  // Ambiguous, depends on the trait used to call the method
    <C as I1>::fun(&c); // Calls the `fun` method from I1
    <C as I2>::fun(&c); // Calls the `fun` method from I2
}
