mod linked_list;
mod stack;
mod queue;
mod binary_tree;
use binary_tree::*;
use linked_list::LinkedList;
fn main() {
    let mut tree = Tree::new();
    tree.insert(1);
    tree.insert(4);
    tree.insert(3);
    tree.insert(5);
    tree.insert(7);
    tree.insert(2);
    tree.insert(10);
    tree.insert(15);


    println!("{:#?}", tree);
}
