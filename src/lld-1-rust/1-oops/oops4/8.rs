// Create a class Node with following requirements

//     It should have two data-members
//         data:int
//         next:Node
//     It should have the following constructors
//         A public constructor that takes an integer and sets the data field
//         A public constructor that takes an integer as well as Node and sets both the data and the next field
//     Implement necessary classes and interfaces to enable for-each loop over Node class.

// Define the Node struct with two fields: data and next

// Define the Node struct with two fields: data and next
#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[allow(dead_code)]
impl Node {
    // Constructor that takes an integer and sets the data field
    pub fn new(data: i32) -> Self {
        Node { data, next: None }
    }

    // Constructor that takes an integer and a Node and sets both the data and the next field
    pub fn new_with_next(data: i32, next: Node) -> Self {
        Node {
            data,
            next: Some(Box::new(next)),
        }
    }

    // Method to append a node to the end of the list
    pub fn append(&mut self, data: i32) {
        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = Some(Box::new(Node::new(data)));
    }
}

// Implement the Iterator trait for Node
impl Iterator for Node {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let node = *node;
            self.data = node.data;
            self.next = node.next;
            self.data
        })
    }
}

fn main() {
    // Create a linked list of nodes
    let mut head = Node::new(1);
    head.append(2);
    head.append(3);

    // Iterate over the list using a for-each loop
    for data in head {
        println!("{}", data);
    }

    // Create individual nodes
    let node6 = Node::new(6);
    let node5 = Node::new_with_next(5, node6);
    let node4 = Node::new_with_next(4, node5);

    // Iterate over the list starting from node1
    let mut current = &node4;
    loop {
        println!("{}", current.data);
        match &current.next {
            Some(next_node) => current = next_node,
            None => break,
        }
    }
}

