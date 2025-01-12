

pub fn bubble_sort<T: Ord>(vec: &mut Vec<T>) {
    for i in 0..vec.len() - 1 {
        for j in 0..vec.len() - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}


pub fn merge_sort<T: Ord + Clone>(vec: &mut Vec<T>) {
    let n = vec.len()/ 2;
    let mut left = &mut vec[..n - 1].to_vec();
    let mut right = &mut vec[n..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(vec, left.as_mut(),right.as_mut())
}


fn merge<T: Ord + Clone> (vec: &mut Vec<T>, left: &mut Vec<T>, right: &mut Vec<T>) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len()  {
        if left[i] <= right[j] {
            vec[k] = left[i].clone();
            i += 1;
        } else {
            vec[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        vec[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len()  {
        vec[k] = right[j].clone();
        j += 1;
        k += 1;
    }

}