// Command Pattern, a behavioral design pattern that encapsulates a request as an object, thereby allowing for parameterization of clients with queues, 
// requests, and operations.

use std::io::{self, Write};

// Command trait with matches and execute functions
trait Command {
    fn matches(&self, input: &str) -> bool;
    fn execute(&self, input: &str);
}

// RegisterUser command
struct RegisterUser;

impl Command for RegisterUser {
    fn matches(&self, input: &str) -> bool {
        input.starts_with("register user")
    }

    fn execute(&self, input: &str) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 3 {
            println!("Usage: register user <username>");
            return;
        }
        let username = parts[2];
        println!("Registering user: {}", username);
        // Add actual logic here
    }
}

// AddUserToGroup command
struct AddUserToGroup;

impl Command for AddUserToGroup {
    fn matches(&self, input: &str) -> bool {
        input.starts_with("add user to group")
    }

    fn execute(&self, input: &str) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 5 {
            println!("Usage: add user to group <username> <groupname>");
            return;
        }
        let username = parts[4];
        let groupname = parts[5];
        println!("Adding user: {} to group: {}", username, groupname);
        // Add actual logic here
    }
}

// CommandExecutor to manage and execute commands
struct CommandExecutor {
    commands: Vec<Box<dyn Command>>,
}

impl CommandExecutor {
    fn new() -> Self {
        CommandExecutor { commands: Vec::new() }
    }

    fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn execute(&self, input: &str) {
        for command in &self.commands {
            if command.matches(input) {
                command.execute(input);
                return;
            }
        }
        println!("Unknown command: {}", input);
    }
}

fn main() {
    let mut executor = CommandExecutor::new();
    executor.add_command(Box::new(RegisterUser));
    executor.add_command(Box::new(AddUserToGroup));

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        executor.execute(input);
    }
}
