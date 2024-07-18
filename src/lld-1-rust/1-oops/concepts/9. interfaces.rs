// In Rust, the concept of interfaces is implemented through traits. Traits define a set of 
// methods and associated types that a type must implement to conform to the interface. Traits can 
// be used for polymorphism, similar to interfaces in other languages like Java or C#.

// Define a trait `Printable` that acts as an interface
trait Printable {
    // Define a method `print` that types implementing this trait must provide
    fn print(&self);
}

// Implement the `Printable` trait for a struct `Book`
struct Book {
    title: String,
    author: String,
}

impl Printable for Book {
    fn print(&self) {
        println!("Book: {} by {}", self.title, self.author);
    }
}

// Implement the `Printable` trait for a struct `Article`
struct Article {
    title: String,
    content: String,
}

impl Printable for Article {
    fn print(&self) {
        println!("Article: {}\n{}", self.title, self.content);
    }
}

// A function that takes a reference to a `Printable` and calls its `print` method
fn print_item(item: &dyn Printable) {
    item.print();
}

fn main() {
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
    };

    let article = Article {
        title: "Traits in Rust".to_string(),
        content: "Traits are a powerful feature in Rust...".to_string(),
    };

    // Use the `print_item` function with different types that implement `Printable`
    print_item(&book);
    print_item(&article);
}
