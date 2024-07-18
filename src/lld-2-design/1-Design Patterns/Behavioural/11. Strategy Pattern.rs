// The Strategy Pattern allows you to define a family of payment algorithms and choose one at runtime based on the user's choice.

// Define the PaymentStrategy trait
trait PaymentStrategy {
    fn pay(&self, amount: f64);
}

// Implement different payment strategies
struct CreditCardPayment;
impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) {
        println!("Processing credit card payment of ${}", amount);
    }
}

struct PayPalPayment;
impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) {
        println!("Processing PayPal payment of ${}", amount);
    }
}

struct CryptoPayment;
impl PaymentStrategy for CryptoPayment {
    fn pay(&self, amount: f64) {
        println!("Processing cryptocurrency payment of ${}", amount);
    }
}

// Context that uses a PaymentStrategy
struct PaymentContext {
    strategy: Box<dyn PaymentStrategy>,
}

impl PaymentContext {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        PaymentContext { strategy }
    }

    fn execute_payment(&self, amount: f64) {
        self.strategy.pay(amount);
    }
}

fn main() {
    let amount = 100.0;

    let credit_card_payment = PaymentContext::new(Box::new(CreditCardPayment));
    credit_card_payment.execute_payment(amount);

    let paypal_payment = PaymentContext::new(Box::new(PayPalPayment));
    paypal_payment.execute_payment(amount);

    let crypto_payment = PaymentContext::new(Box::new(CryptoPayment));
    crypto_payment.execute_payment(amount);
}
