use std::mem;

// Define a struct named 'Student'
struct Student {
    age: Option<i32>,
    name: Option<String>,
}

// Implementation block for 'Student'
impl Student {
    // Method to display a student's details
    fn display(&self) {
        let binding = "Unknown".to_string();
        let name: &String = self.name.as_ref().unwrap_or(&binding);
        let age: i32 = self.age.unwrap_or(-1); // Using -1 or any other value to indicate unknown age

        println!("My name is {}. I am {} years old", name, age);
    }

    // Method for a student to say hello to someone
    #[allow(dead_code)]
    fn say_hello(&self, other_name: &str) {
        // Prints a greeting from this student to another person
        let binding = "Unknown".to_string();
        let name: &String = self.name.as_ref().unwrap_or(&binding);
        println!("{} says hello to {}", name, other_name);
    }
}

// The main function - entry point of the program
fn main() {
    // Create an instance of 'Student' with age 10 and name "A"
    #[allow(unused_variables)]
    let mut s1: Student = Student {
        age: Some(10),
        name: Some("A".to_string()),
    };

    // Create an instance of 'Student' with age 20 and name "B"
    let mut s2: Student = Student {
        age: Some(20),
        name: Some("B".to_string()),
    };

    swap(&mut s1, &mut s2);

    // Display the details of student 's2' (which are now modified)
    s1.display();
}

fn swap(s1: &mut Student, s2: &mut Student) {
    mem::swap(&mut s1.age, &mut s2.age);
    mem::swap(&mut s1.name, &mut s2.name);
}

// Output:
// My name is B. I am 20 years old