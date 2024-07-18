// Create a Student class satisfying following requirements

// It should have two data members:
//     age: int
//     name: String

// It should have a display method
//     Signature - void:display()
//     It should print: “My name is <name>. I am <age> years old”

// It should have a sayHello method
//     Signature - void:sayHello(String)
//     It should print “<name data member> says hello to <name parameter>”

struct Student {
    age: i32,
    name: String,
}

impl Student {
    // Return details as a String instead of printing
    fn display(&self) -> String {
        format!("My name is {}. I am {} years old", self.name, self.age)
    }

    // Return greeting as a String
    fn say_hello(&self, other_name: &str) -> String {
        format!("{} says hello to {}", self.name, other_name)
    }
}

fn main() {
    let student: Student = Student {
        age: 20,
        name: "Alice".to_string(),
    };

    student.display();
    student.say_hello("Bob");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let student = Student {
            age: 20,
            name: "Alice".to_string(),
        };

        let output = student.display();
        assert_eq!(output, "My name is Alice. I am 20 years old");
    }

    #[test]
    fn test_say_hello() {
        let student = Student {
            age: 20,
            name: "Alice".to_string(),
        };

        let output = student.say_hello("Bob");
        assert_eq!(output, "Alice says hello to Bob");
    }
}

