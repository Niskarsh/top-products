#![allow(non_snake_case)]

#[derive(Clone, Debug)]
struct Node {
    element: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(element: &i32) -> Self {
        Node { element: *element, next: None }
    }
}
#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn init() -> Self {
        LinkedList { head: None }
    }

    pub fn add(&mut self, element: &i32) {
        let mut newNode = Node::new(element);
        // Null LL
        match self.head.clone() {
            None => self.head = Some(Box::new(newNode)),
            Some(old_head) => {
                newNode.next = Some(old_head);
                self.head = Some(Box::new(newNode));
            }
        }
    }
}