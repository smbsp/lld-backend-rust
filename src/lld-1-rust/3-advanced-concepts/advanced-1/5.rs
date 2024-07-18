// Use enhanced for-loop on the objects of Node

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32, next: Option<Box<Node>>) -> Self {
        Node { data, next }
    }
}

struct NodeIterator {
    current: Option<Box<Node>>,
}

impl IntoIterator for Node {
    type Item = i32;
    type IntoIter = NodeIterator;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator { current: Some(Box::new(self)) }
    }
}

impl Iterator for NodeIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next;
            node.data
        })
    }
}

fn main() {
    let list = Node::new(1, Some(Box::new(Node::new(2, Some(Box::new(Node::new(3, None)))))));

    for data in list {
        println!("{}", data);
    }
}
