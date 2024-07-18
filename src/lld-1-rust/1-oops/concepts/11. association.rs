// Association in Rust refers to a relationship between types where one type is connected to 
// another, but they are not dependent on each other for their existence. This is different from 
// composition, where one type owns another type. In association, the associated types can exist independently.

// Define a struct `Author` with some properties
struct Author {
    name: String,
    age: u32,
}

// Define a struct `Book` with some properties and an associated `Author`
struct Book<'a> {
    title: String,
    author: &'a Author, // Association: a reference to an `Author`
}

impl Author {
    // Constructor for `Author`
    fn new(name: String, age: u32) -> Self {
        Author { name, age }
    }
}

impl<'a> Book<'a> {
    // Constructor for `Book` that takes a reference to an `Author`
    fn new(title: String, author: &'a Author) -> Self {
        Book { title, author }
    }

    // Method to display information about the book
    fn display_info(&self) {
        println!("Title: {}", self.title);
        println!("Author: {}, Age: {}", self.author.name, self.author.age);
    }
}

fn main() {
    let author = Author::new("J.K. Rowling".to_string(), 55);
    let book = Book::new("Harry Potter and the Sorcerer's Stone".to_string(), &author);

    book.display_info();
}
