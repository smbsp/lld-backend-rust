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
    let mut s1: Student = Student {
        age: 10,
        name: "A".to_string(),
    };
    // Display the details of student 's1'
    s1.display();

    // Create a mutable reference 's2' to 's1'
    let s2: &mut Student = &mut s1;
    // Change the age and name of the student via the mutable reference 's2'
    s2.age = 20;
    s2.name = "B".to_string();

    // Display the details of student 's2' (which are now modified)
    s2.display();

    // Display the details of student 's1' again
    // This will show the modified details as 's2' was a reference to 's1'
    s1.display();
}

// Output:
// My name is A. I am 10 years old
// My name is B. I am 20 years old
// My name is B. I am 20 years old