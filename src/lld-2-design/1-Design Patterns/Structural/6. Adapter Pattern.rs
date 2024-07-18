// Adapting a Third-Party Logging Library

// Third-party logging library
struct ExternalLogger;

impl ExternalLogger {
    fn external_log(&self, msg: &str) {
        println!("ExternalLogger: {}", msg);
    }
}

// Our application's logging trait
trait AppLogger {
    fn log(&self, msg: &str);
}

// Adapter for ExternalLogger
struct LoggerAdapter {
    external_logger: ExternalLogger,
}

impl LoggerAdapter {
    fn new(external_logger: ExternalLogger) -> Self {
        LoggerAdapter { external_logger }
    }
}

impl AppLogger for LoggerAdapter {
    fn log(&self, msg: &str) {
        self.external_logger.external_log(msg);
    }
}

fn main() {
    let external_logger = ExternalLogger;
    let logger_adapter = LoggerAdapter::new(external_logger);

    // Use the adapter as if it's the application's logger
    logger_adapter.log("This is a message.");
}
