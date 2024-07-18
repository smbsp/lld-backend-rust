// In a web application, you might have several services (e.g., logging, database access) that need to be accessible 
// throughout the application. The Registry Pattern can be used to implement a Service Locator.

// Note: By using RwLock, we ensure that the ServiceRegistry can be safely accessed and modified by multiple threads concurrently.

use std::collections::HashMap;
use std::sync::Arc;

trait Service {
    fn execute(&self);
}

struct LoggerService;
impl Service for LoggerService {
    fn execute(&self) {
        println!("Logging...");
    }
}

struct DatabaseService;
impl Service for DatabaseService {
    fn execute(&self) {
        println!("Accessing database...");
    }
}

// dyn Service indicates a trait object. This means that Service is a trait, and dyn Service is a way to use dynamic dispatch 
// to call methods on any type that implements the Service trait.
// Send indicates that it is safe to transfer the ownership of the type to another thread.
// Sync indicates that it is safe to reference the type from multiple threads concurrently.
struct ServiceRegistry {
    services: HashMap<String, Arc<dyn Service + Send + Sync>>,
}

impl ServiceRegistry {
    fn new() -> Self {
        ServiceRegistry {
            services: HashMap::new(),
        }
    }

    fn register_service(&mut self, name: String, service: Arc<dyn Service + Send + Sync>) {
        self.services.insert(name, service);
    }

    fn get_service(&self, name: &str) -> Option<Arc<dyn Service + Send + Sync>> {
        self.services.get(name).cloned()
    }
}

fn main() {
    let mut registry = ServiceRegistry::new();

    registry.register_service(
        "logger".to_string(),
        Arc::new(LoggerService),
    );

    registry.register_service(
        "database".to_string(),
        Arc::new(DatabaseService),
    );

    if let Some(service) = registry.get_service("logger") {
        service.execute();
    }

    if let Some(service) = registry.get_service("database") {
        service.execute();
    }
}
