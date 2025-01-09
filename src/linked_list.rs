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

 impl<T: std::cmp::PartialEq + std::fmt::Debug + std::fmt::Display > LinkedList<T> {
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


