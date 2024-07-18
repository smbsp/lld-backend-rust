// Adding features such as timestamping, log level filtering without modifying the original logging class.

use std::time::SystemTime;

// Define the Logger trait
trait Logger {
    fn log(&self, msg: &str);
}

// Basic console logger
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

// Decorator for adding timestamps
struct TimestampLogger<T: Logger> {
    logger: T,
}

impl<T: Logger> TimestampLogger<T> {
    fn new(logger: T) -> Self {
        TimestampLogger { logger }
    }
}

impl<T: Logger> Logger for TimestampLogger<T> {
    fn log(&self, msg: &str) {
        let now = SystemTime::now();
        self.logger.log(&format!("{:?}: {}", now, msg));
    }
}

// Decorator for adding log levels
struct LevelLogger<T: Logger> {
    logger: T,
    level: String,
}

impl<T: Logger> LevelLogger<T> {
    fn new(logger: T, level: String) -> Self {
        LevelLogger { logger, level }
    }
}

impl<T: Logger> Logger for LevelLogger<T> {
    fn log(&self, msg: &str) {
        self.logger.log(&format!("[{}] {}", self.level, msg));
    }
}

fn main() {
    let console_logger = ConsoleLogger;
    let timestamp_logger = TimestampLogger::new(console_logger);
    let level_logger = LevelLogger::new(timestamp_logger, "INFO".to_string());

    level_logger.log("This is a log message with level and timestamp.");
}
