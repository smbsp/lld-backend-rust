// In a game, characters often have many attributes (e.g., health, stamina, strength, etc.). Using the Prototype Pattern, 
// you can create a base character and then clone it to create customized versions.

#[derive(Clone, Debug)]
struct GameCharacter {
    name: String,
    health: u32,
    stamina: u32,
    strength: u32,
}

impl GameCharacter {
    fn new(name: &str, health: u32, stamina: u32, strength: u32) -> Self {
        GameCharacter {
            name: name.to_string(),
            health,
            stamina,
            strength,
        }
    }

    fn clone_character(&self) -> Self {
        self.clone()
    }

    // Method to describe the character
    fn describe(&self) {
        println!(
            "Name: {}, Health: {}, Stamina: {}, Strength: {}",
            self.name, self.health, self.stamina, self.strength
        );
    }
}

fn main() {
    let warrior = GameCharacter::new("Warrior", 100, 75, 80);
    let mut cloned_warrior = warrior.clone_character();
    cloned_warrior.name = "Warrior Clone".to_string();

    println!("Original: {:?}", warrior);
    warrior.describe();
    println!("Clone: {:?}", cloned_warrior);
    cloned_warrior.describe();
}
