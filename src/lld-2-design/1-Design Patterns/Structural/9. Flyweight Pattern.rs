// Flyweight Pattern - Reduces memory usage by sharing common parts of the state (intrinsic state) between multiple objects.
// In this case, TreeType objects are shared among multiple Tree instances.

use std::collections::HashMap;
use std::rc::Rc;

// Flyweight object
struct TreeType {
    name: String,
    color: String,
    texture: String,
}

impl TreeType {
    fn new(name: &str, color: &str, texture: &str) -> Self {
        TreeType {
            name: name.to_string(),
            color: color.to_string(),
            texture: texture.to_string(),
        }
    }

    fn draw(&self, x: i32, y: i32) {
        println!(
            "Drawing tree of type '{}' with color '{}' and texture '{}' at position ({}, {})",
            self.name, self.color, self.texture, x, y
        );
    }
}

// Flyweight factory
struct TreeFactory {
    tree_types: HashMap<String, Rc<TreeType>>,
}

impl TreeFactory {
    fn new() -> Self {
        TreeFactory {
            tree_types: HashMap::new(),
        }
    }

    fn get_tree_type(&mut self, name: &str, color: &str, texture: &str) -> Rc<TreeType> {
        let key = format!("{}_{}_{}", name, color, texture);
        if !self.tree_types.contains_key(&key) {
            let tree_type = Rc::new(TreeType::new(name, color, texture));
            self.tree_types.insert(key.clone(), tree_type);
        }
        self.tree_types.get(&key).unwrap().clone()
    }
}

// Context object
struct Tree {
    x: i32,
    y: i32,
    tree_type: Rc<TreeType>,
}

impl Tree {
    fn new(x: i32, y: i32, tree_type: Rc<TreeType>) -> Self {
        Tree { x, y, tree_type }
    }

    fn draw(&self) {
        self.tree_type.draw(self.x, self.y);
    }
}

fn main() {
    let mut tree_factory = TreeFactory::new();

    let pine_tree_type = tree_factory.get_tree_type("Pine", "Green", "Rough");
    let oak_tree_type = tree_factory.get_tree_type("Oak", "Brown", "Smooth");

    let trees = vec![
        Tree::new(10, 20, pine_tree_type.clone()),
        Tree::new(30, 40, pine_tree_type.clone()),
        Tree::new(50, 60, oak_tree_type.clone()),
    ];

    for tree in trees {
        tree.draw();
    }
}
