// Used to notify multiple objects of a state change. A real-life example is an event handling system where multiple components react to events.

use std::cell::RefCell;
use std::rc::Rc;

// Define the Observer trait
trait Observer {
    fn update(&self, message: &str);
}

// Concrete observer implementations
struct Logger;
impl Observer for Logger {
    fn update(&self, message: &str) {
        println!("Logger: {}", message);
    }
}

struct Notifier;
impl Observer for Notifier {
    fn update(&self, message: &str) {
        println!("Notifier: {}", message);
    }
}

// Subject that maintains a list of observers
struct EventManager {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl EventManager {
    fn new() -> Self {
        EventManager { observers: Vec::new() }
    }

    fn subscribe(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.borrow().update(message);
        }
    }
}

fn main() {
    let logger = Rc::new(RefCell::new(Logger));
    let notifier = Rc::new(RefCell::new(Notifier));

    let mut event_manager = EventManager::new();
    event_manager.subscribe(logger.clone());
    event_manager.subscribe(notifier.clone());

    event_manager.notify("An important event occurred.");
}
