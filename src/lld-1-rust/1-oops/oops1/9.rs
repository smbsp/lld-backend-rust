// It should have 3 data members
//     accountNumber: String
//     balance: int
//     roi:double (Should represent rate of interest)

// It should have 2 methods
//     getSimpleInterest: It should take time (in years) as a parameter. The data type of time should be int. It should return Simple Interest as a double.
//     getBalanceWithInterest: It should take time (in years) as a parameter. The data type of time should be int. It should return a new balance (including simple interest) as a double.

use std::u8;

#[allow(dead_code)]
struct Account {
    account_number: String,
    balance: i32,
    roi: f64,
}

impl Account {
    fn get_simple_interest(&self, time: u8) -> f64 {
        (self.balance as f64) * (self.roi / 100.0) * (time as f64)
    }

    fn get_balance_with_interest(&self, time: u8) -> f64 {
        (self.balance as f64) + self.get_simple_interest(time)
    }
}

fn main() {
    let account: Account = Account {
        account_number: "123456789".to_string(),
        balance: 1000,
        roi: 3.5, // 3.5% interest rate
    };

    let time = 5; // 5 years
    println!("Simple Interest: {}", account.get_simple_interest(time));
    println!("Balance With Interest: {}", account.get_balance_with_interest(time));
}