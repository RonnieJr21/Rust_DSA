
pub fn binary_search<T: PartialEq + PartialOrd>(arr: &Vec<T>, target: &T) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len();
    while l < r {
        let m = (l + r) / 2;
        if arr[m] == *target{
        return Some(m)
        } else if arr[m] < *target {
            l = m + 1;
        } else {
            r = m;
        }
    }
   None
}