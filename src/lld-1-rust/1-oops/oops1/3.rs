// Define a struct named 'Student'
struct Student {
    age: i32,       // Field 'age' of type i32
    name: String,   // Field 'name' of type String
}

// Implementation block for 'Student'
impl Student {
    // Method to display a student's details
    fn display(&self) {
        // Prints the student's name and age to the console
        println!("My name is {}. I am {} years old", self.name, self.age);
    }

    // Method for a student to say hello to someone
    #[allow(dead_code)]
    fn say_hello(&self, name: &str) {
        // Prints a greeting from this student to another person
        println!("{} says hello to {}", self.name, name);
    }
}

// The main function - entry point of the program
fn main() {
    // Create an instance of 'Student' with age 10 and name "A"
    #[allow(unused_variables)]
    let s1: Student = Student {
        age: 10,
        name: "A".to_string(),
    };

    // Create an instance of 'Student' with age 20 and name "B"
    let s2: Student = Student {
        age: 20,
        name: "B".to_string(),
    };

    // Display the details of student 's2' (which are now modified)
    s2.display();
}

// Output:
// My name is B. I am 20 years old