use std::cmp;

/// Calculate [Z-function](https://cp-algorithms.com/string/z-function.html)
/// of a given slice of an [`Eq`] type `T`.
///
/// # Examples
///
/// ```
/// use cp_library::string::z_array;
///
/// let arr = "anananasso";
/// let z = z_array(arr.as_bytes());
///
/// assert_eq!(z, [10, 0, 5, 0, 3, 0, 1, 0, 0, 0]);
/// ```
pub fn z_array<T: Eq>(arr: &[T]) -> Vec<usize> {
    let mut z = vec![0; arr.len()];
    let mut l = 0;
    let mut r = 0;

    z[0] = arr.len();
    for i in 1..arr.len() {
        if i <= r {
            z[i] = cmp::min(r - i, z[i - l]);
        }
        while i + z[i] < z.len() && arr[i + z[i]] == arr[z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }

    z
}
