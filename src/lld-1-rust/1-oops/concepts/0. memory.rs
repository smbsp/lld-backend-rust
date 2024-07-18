fn main() {
    let x = 42; // `x` is stored on the stack
    let y = &x; // `y` is a reference to `x`, storing the memory address of `x`

    println!("Value of x: {}", x); // Outputs: 42
    println!("Address of x: {:p}", y); // Outputs the memory address of `x`

    let vec = vec![1, 2, 3]; // `vec` is stored on the heap, and `vec` itself is a pointer stored on the stack
    let vec_address = &vec; // `vec_address` is a reference to `vec`, storing the memory address of the pointer

    println!("Value of vec: {:?}", vec); // Outputs: [1, 2, 3]
    println!("Address of vec pointer: {:p}", vec_address); // Outputs the memory address of the `vec` pointer

    // let vec = vec![1, 2, 3]; // `vec` is a pointer to heap memory
    let first_element_address = &vec[0]; // Get the address of the first element of the vector

    println!("Address of the first element on the heap: {:p}", first_element_address);
}