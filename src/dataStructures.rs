#[derive(Debug)]
struct TNode<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> TNode<T> {
    fn new(data: T) -> Self {
        TNode {data, left:None,right:None}
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
    fn insert_recurse(node: &mut TNode<T>, data: T) {
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



pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> { // will need to be unwrapped when used
        Some(self.data.remove(0))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_full(&self) -> bool {
        self.size() == self.data.len()
    }

    pub fn get_first(&self) -> Option<&T> { // will need to be unwrapped when used
        self.data.first()
    }

    pub fn get_last(&self) -> Option<&T> { // will need to be unwrapped when used
        self.data.last()
    }
}

#[derive(PartialEq)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {value, next: None}
    }
}

#[derive(PartialEq)]

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq + std::fmt::Debug + std::fmt::Display > LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None}
    }

    pub fn start(&mut self, value: T) {
        self.head = Some(Box::new(Node::new(value)));
    }

    pub fn append(&mut self, value: T) {
        let mut n = &mut self.head;
        while let Some (ref mut node) = n {
            n = &mut node.next;
        }
        *n = Some(Box::new(Node::new(value)))
    }

    pub fn show(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}



struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> { // will need to be unwrapped when used
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn size(&self) -> usize {
        self.data.len()
    }

    fn get_last(&self) -> Option<&T> { // will need to be unwrapped when used
        self.data.last()
    }
}


