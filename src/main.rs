mod linked_list;
mod stack;
mod queue;
mod binary_tree;
mod binary_search;
mod sorting;
use sorting::*;

use binary_search::binary_search;
use binary_tree::*;
fn main() {
    let sorted_array:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    let mut unsorted_array:Vec<i32> = vec![39, 62, 58, 48, 53, 93, 41, 2, 95, 40, 77, 34, 63, 74, 86, 22, 41, 41, 98, 1];
    let target = 7;
    bubble_sort(unsorted_array.as_mut());

    println!("{:?}", unsorted_array);


}
