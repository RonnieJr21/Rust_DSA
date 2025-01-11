

pub fn bubble_sort<T: Ord>(vec: &mut Vec<T>) {
    for i in 0..vec.len() - 1 {
        for j in 0..vec.len() - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}