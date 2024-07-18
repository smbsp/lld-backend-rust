// Dependency Inversion Principle (DIP) - In this example, the Application struct depends on the Database trait, not a specific database implementation, adhering to the DIP.

trait Database {
    fn save(&self, data: &str);
}

struct MySqlDatabase;

impl Database for MySqlDatabase {
    fn save(&self, data: &str) {
        println!("Saving '{}' to MySQL database", data);
    }
}

struct Application<D: Database> {
    db: D,
}

impl<D: Database> Application<D> {
    fn new(db: D) -> Self {
        Application { db }
    }

    fn save_data(&self, data: &str) {
        self.db.save(data);
    }
}

fn main() {
    let db = MySqlDatabase;
    let app = Application::new(db);
    app.save_data("User data");
}
