// Interface Segregation Principle (ISP) - In this example, SimplePrinter only implements the Printer trait, adhering to the ISP.

trait Printer {
    fn print(&self);
}

trait Scanner {
    fn scan(&self);
}

struct AllInOnePrinter;

impl Printer for AllInOnePrinter {
    fn print(&self) {
        println!("Printing document");
    }
}

impl Scanner for AllInOnePrinter {
    fn scan(&self) {
        println!("Scanning document");
    }
}

struct SimplePrinter;

impl Printer for SimplePrinter {
    fn print(&self) {
        println!("Printing document");
    }
}

fn main() {
    let all_in_one = AllInOnePrinter;
    let simple_printer = SimplePrinter;

    all_in_one.print();
    all_in_one.scan();

    simple_printer.print();
}
