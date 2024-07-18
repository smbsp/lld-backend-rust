// Define a struct `BankAccount` with private fields
struct BankAccount {
    balance: f64,
}

impl BankAccount {
    // Constructor method to create a new `BankAccount`
    pub fn new(initial_balance: f64) -> Self {
        BankAccount {
            balance: initial_balance,
        }
    }

    // Public method to get the current balance
    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    // Public method to deposit money into the account
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    // Public method to withdraw money from the account
    pub fn withdraw(&mut self, amount: f64) -> bool {
        if amount > 0.0 && self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut account = BankAccount::new(100.0);
    println!("Initial balance: {}", account.get_balance());

    account.deposit(50.0);
    println!("Balance after deposit: {}", account.get_balance());

    if account.withdraw(30.0) {
        println!("Balance after withdrawal: {}", account.get_balance());
    } else {
        println!("Insufficient funds");
    }
}
