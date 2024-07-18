// Write code to achieve the following

//     A class Node to represent the Node of a BinaryTree.
//     Node should have two properties - left and right of type Node.
//     Node should also have an integer property - data.
//     Create a class TreeSizeCalculator to calculate the size of a BinaryTree using multiple threads.
//     TreeSizeCalculator constructor will be passed the root of the tree and ExecutorService to make new threads.
//     TreeSizeCalculator should implement Callable<Integer> interface

// Arc<T>: Stands for Atomic Reference Counting. It's a thread-safe way of sharing ownership of data (T) 
// across multiple threads. When the last reference to the data is dropped, the data is automatically deallocated.

// Mutex<T>: Provides mutual exclusion, meaning it ensures that only one thread at a time can access the 
// data (T) it protects. You lock the mutex to access the data, and other threads are blocked until 
// the mutex is unlocked.
use std::sync::{Arc, Mutex};

#[allow(dead_code)]

// The children are wrapped in Arc<Mutex<_>> to allow safe shared access from multiple threads.
#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Arc<Mutex<Node>>>,
    right: Option<Arc<Mutex<Node>>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

struct TreeSizeCalculator {
    // The root node is wrapped in Arc<Mutex<_>> to allow safe shared access from multiple threads.
    root: Arc<Mutex<Node>>,
}

impl TreeSizeCalculator {
    fn new(root: Arc<Mutex<Node>>) -> Self {
        TreeSizeCalculator { root }
    }

    fn calculate_size(&self) -> usize {
        fn size(node: &Option<Arc<Mutex<Node>>>) -> usize {
            match node {
                Some(n) => {
                    let n = n.lock().unwrap();
                    1 + size(&n.left) + size(&n.right)
                }
                None => 0,
            }
        }

        size(&Some(Arc::clone(&self.root)))
    }
}

#[allow(unused_must_use)]
fn main() {
    // Create a sample binary tree
    let root = Arc::new(Mutex::new(Node::new(1)));
    let left = Arc::new(Mutex::new(Node::new(2)));
    let right = Arc::new(Mutex::new(Node::new(3)));

    // the new scope is used to manage the lifetime of the lock on the Node
    {
        let mut root = root.lock().unwrap();
        root.left = Some(Arc::clone(&left));
        root.right = Some(Arc::clone(&right));
        // dbg!(&root.left);
    }

    // Calculate the size of the binary tree using TreeSizeCalculator
    let calculator = TreeSizeCalculator::new(Arc::clone(&root));
    let size = calculator.calculate_size();
    println!("Size of the binary tree: {}", size);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_size() {
        // Create the nodes of the binary tree
        let root = Arc::new(Mutex::new(Node::new(1)));
        let left = Arc::new(Mutex::new(Node::new(2)));
        let right = Arc::new(Mutex::new(Node::new(3)));
        let left_left = Arc::new(Mutex::new(Node::new(4)));
        let left_right = Arc::new(Mutex::new(Node::new(5)));
        let right_left = Arc::new(Mutex::new(Node::new(6)));
        let right_right = Arc::new(Mutex::new(Node::new(7)));

        // Build the tree structure
        {
            let mut root = root.lock().unwrap();
            root.left = Some(Arc::clone(&left));
            root.right = Some(Arc::clone(&right));

            let mut left = left.lock().unwrap();
            left.left = Some(Arc::clone(&left_left));
            left.right = Some(Arc::clone(&left_right));

            let mut right = right.lock().unwrap();
            right.left = Some(Arc::clone(&right_left));
            right.right = Some(Arc::clone(&right_right));
        }

        // Calculate the size of the binary tree
        let calculator = TreeSizeCalculator::new(Arc::clone(&root));
        let size = calculator.calculate_size();

        // Assert that the size is correct
        assert_eq!(size, 7);
    }
}
