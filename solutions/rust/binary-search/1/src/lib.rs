pub fn find<T, F>(array: F, key: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
    F: AsRef<[T]>,
{
    let array = array.as_ref();
    let mut l = 0;
    let mut r = array.len();

    while l < r {
        let mid = l + (r - l) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            l = mid + 1;
        } else {
            r = mid;
        }
    }

    
    None
}