// We define two functions, add and subtract, with the same signature (i32, i32) -> i32. We then create function pointers 
// add_fn and subtract_fn to store references to these functions. The type of these function pointers is fn(i32, i32) -> i32, 
// which matches the signature of the functions they point to.

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    // Assign function pointers to variables
    let add_fn: fn(i32, i32) -> i32 = add;
    let subtract_fn: fn(i32, i32) -> i32 = subtract;

    // Use function pointers to call functions
    let sum = add_fn(5, 3);
    let difference = subtract_fn(10, 4);

    println!("Sum: {}", sum);         // Output: Sum: 8
    println!("Difference: {}", difference); // Output: Difference: 6
}
