// This Rust code defines functions to find the smallest element greater than or equal to a given value (get_just_larger) 
// and the largest element less than or equal to a given value (get_just_smaller) in a BTreeSet

use std::collections::BTreeSet;

fn get_just_larger(tree_set: &BTreeSet<i32>, data: i32) -> Option<i32> {
    tree_set.range(data..).next().copied()
}

fn get_just_smaller(tree_set: &BTreeSet<i32>, data: i32) -> Option<i32> {
    tree_set.range(..=data).next_back().copied()
}

fn main() {
    let mut tree_set = BTreeSet::new();
    tree_set.insert(1);
    tree_set.insert(3);
    tree_set.insert(5);
    tree_set.insert(7);

    let just_larger = get_just_larger(&tree_set, 4);
    let just_smaller = get_just_smaller(&tree_set, 4);

    println!("Just larger than 4: {:?}", just_larger); // Should print Some(5)
    println!("Just smaller than 4: {:?}", just_smaller); // Should print Some(3)
}
