
mod sorting;
use sorting::*;
mod dataStructures;
use dataStructures::*;
mod binary_search;

fn main() {
    let mut unsorted_array:Vec<i32> = vec![39, 62, 58, 48, 53, 93, 41, 2, 95, 40, 77, 34, 63, 74, 86, 22, 41, 41, 98, 1];
    bubble_sort(&mut unsorted_array);

    println!("{:?}", unsorted_array);


}
