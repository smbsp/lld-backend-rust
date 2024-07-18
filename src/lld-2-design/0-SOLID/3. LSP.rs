// Liskov Substitution Principle (LSP) - Here, both Sparrow and Penguin can be used where Bird is expected, following the LSP.

trait Bird {
    fn fly(&self);
}

struct Sparrow;

impl Bird for Sparrow {
    fn fly(&self) {
        println!("Sparrow is flying");
    }
}

struct Penguin;

impl Bird for Penguin {
    fn fly(&self) {
        // Penguins can't fly, so we handle it differently
        println!("Penguin is swimming");
    }
}

fn let_bird_fly(bird: &dyn Bird) {
    bird.fly();
}

fn main() {
    let sparrow = Sparrow;
    let penguin = Penguin;

    let_bird_fly(&sparrow);
    let_bird_fly(&penguin);
}
