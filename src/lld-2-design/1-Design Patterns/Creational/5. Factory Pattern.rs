// In a logging library, different types of loggers might be needed depending on the deployment environment or specific logging requirements. 
// The Factory Pattern can be used to encapsulate the creation logic and provide a flexible way to create different logger instances.

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

// Define the Logger trait
trait Logger {
    fn log(&self, message: &str);
}

// Implement a Console Logger
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("ConsoleLogger: {}", message);
    }
}

// Implement a File Logger
struct FileLogger {
    file: Mutex<std::fs::File>,
}

impl FileLogger {
    fn new(file_path: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .unwrap();
        FileLogger {
            file: Mutex::new(file),
        }
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "FileLogger: {}", message).unwrap();
    }
}

// Define the LoggerFactory
struct LoggerFactory;

impl LoggerFactory {
    fn create_logger(logger_type: &str) -> Box<dyn Logger> {
        match logger_type {
            "console" => Box::new(ConsoleLogger),
            "file" => Box::new(FileLogger::new("src/logfile.log")),
            _ => panic!("Unknown logger type"),
        }
    }
}

fn main() {
    // Create a console logger using the factory
    let console_logger = LoggerFactory::create_logger("console");
    console_logger.log("This is a message to the console logger");

    // Create a file logger using the factory
    let file_logger = LoggerFactory::create_logger("file");
    file_logger.log("This is a message to the file logger");
}
