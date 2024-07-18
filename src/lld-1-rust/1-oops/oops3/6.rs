// Write a method display such that

//     We can pass all data-types that supports looping over its data
//     Write the code to print individual items of the passed data over separate lines
//     Syntax requirements on display method
//         display method should be static
//         display method’s return type should be void
//         display method will take one parameter of appropriate type that allows it to loop over data

struct Utility;

impl Utility {
    // Static method `display` that takes a generic parameter `T` constrained by `IntoIterator`
    fn display<T: IntoIterator>(data: T) where <T as IntoIterator>::Item: std::fmt::Display {
        // Loop over the items in `data` and print each item on a separate line
        for item in data {
            println!("{}", item);
        }
    }
}

fn main() {
    // Example usage with a Vec
    let vec = vec![1, 2, 3, 4, 5];
    Utility::display(vec);

    // Example usage with an array
    let arr = [10, 20, 30, 40, 50];
    Utility::display(arr);

    // Example usage with a slice
    let slice = &["a", "b", "c", "d"];
    Utility::display(slice);
}
