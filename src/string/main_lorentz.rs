use super::z_array;

/// A range of square substrings
#[derive(Debug)]
pub struct MainLorentzRepetition {
    /// The length of the substrings
    pub length: usize,
    /// The first valid starting point
    pub l: usize,
    /// The last valid starting point
    pub r: usize,
}

fn main_lorentz_helper<T: Eq>(arr: &[T], l: usize, r: usize, reps: &mut Vec<MainLorentzRepetition>) {
    if r - l <= 1 {
        return;
    }

    let m = (l + r) / 2;
    
    let z_u = z_array(&arr[l..m].iter().rev().collect::<Vec<_>>());
    let z_vu = z_array(&arr[m..r].iter().chain(arr[l..m].iter()).collect::<Vec<_>>());
    let z_v = z_array(&arr[m..r]);
    let z_uv = z_array(&arr[l..m].iter().rev().chain(arr[m..r].iter().rev()).collect::<Vec<_>>());

    for center in l..m {
        let length = m - center;
        let k1 = *z_u.get(m - center).unwrap_or(&0);
        let k2 = z_vu[r - m + center - l];

        if arr[center] == arr[m] && k1 + k2 >= length {
            reps.push(MainLorentzRepetition {
                length: 2 * length,
                l: center - k1.min(length - 1),
                r: center + k2.min(length) - length,
            });
        }
    }

    for center in m..r {
        let length = center - m + 1;
        let k1 = *z_v.get(center - m + 1).unwrap_or(&0);
        let k2 = z_uv[m - l + r - center - 1].min(m - l);

        if arr[center] == arr[m - 1] && k1 + k2 >= length {
            reps.push(MainLorentzRepetition {
                length: 2 * length,
                l: center - k2.min(length) + 1 - length,
                r: center + k1.min(length - 1) + 1 - 2 * length,
            })
        }
    }

    main_lorentz_helper(arr, l, (l + r) / 2, reps);
    main_lorentz_helper(arr, (l + r) / 2, r, reps);
}
 
/// Calculate all squared substrings (in compressed format)
/// of a given slice of an [`Eq`] type `T`.
///
/// A squared string is a string which can be obtained by concatenating two identical strings.
/// 
/// *Warning*: This is not thoroughly tested since there are no (easily accessible)
/// online judges with a task suitable for testing.
/// 
/// Complexity: $\mathcal{O}(N \log N)$ comparisons where:
/// - $N$ is the size of the slice.
pub fn main_lorentz<T: Eq>(arr: &[T]) -> Vec<MainLorentzRepetition> {
    let mut reps = vec![];
    main_lorentz_helper(arr, 0, arr.len(), &mut reps);
    reps
}
