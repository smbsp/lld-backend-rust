// Single Responsibility Principle (SRP) - In this example, Logger is responsible only for logging, and OrderProcessor is responsible only for processing orders.

struct Logger;

impl Logger {
    fn log(&self, message: &str) {
        println!("Log: {}", message);
    }
}

struct OrderProcessor {
    logger: Logger,
}

impl OrderProcessor {
    fn process_order(&self, order_id: u32) {
        self.logger.log(&format!("Processing order: {}", order_id));
        // Order processing logic
    }
}

fn main() {
    let logger = Logger;
    let order_processor = OrderProcessor { logger };
    order_processor.process_order(123);
}
