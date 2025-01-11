use crate::queue::*;
#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {data, left:None,right:None}
    }
}
#[derive(Debug)]
pub struct Tree<T> {
    head: Option<Box<Node<T>>>,
}

impl<T:PartialOrd> Tree<T> {
    pub fn new() -> Self {
        Tree {head:None}
    }
    pub fn insert(&mut self, data:T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(data)));
        }
        else {
            Self::insert_recurse(self.head.as_mut().unwrap(),data);
        }

    }
    fn insert_recurse(node: &mut Node<T>, data: T) {
        let mut queue = Queue::new();
        queue.enqueue(node);
        while let Some(node) = queue.dequeue() {
            if node.left.is_none() {
                node.left = Some(Box::new(Node::new(data)));
                break;
            } else {
                queue.enqueue(node.left.as_mut().unwrap());
            }
            if node.right.is_none(){
                node.right = Some(Box::new(Node::new(data)));
                break;
            } else {
                queue.enqueue(node.right.as_mut().unwrap());
            }
        }
    }
}