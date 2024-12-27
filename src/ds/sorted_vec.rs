use std::convert::From;
use std::ops::{Deref, DerefMut};

use crate::binsearch::first_true;

/// A sorted vector over a total order (`Ord`) `T`
#[derive(Clone, Debug)]
pub struct SortedVec<T> {
    arr: Vec<T>,
}

impl<T> SortedVec<T>
where
    T: Ord + Clone,
{
    /// Conversion from an already sorted slice
    ///
    /// Complexity: $\mathcal{O}(N)$ copies where
    /// - $N$ is the length of the slice
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::ds::sorted_vec::SortedVec;
    ///
    /// let arr = SortedVec::from_sorted_slice(&[1, 2, 2, 3]);
    /// assert_eq!(*arr, vec![1, 2, 2, 3]);
    /// ```
    ///
    /// # Panics
    ///
    /// Only in debug builds, if the slice is not sorted
    /// ```should_panic
    /// use cp_library::ds::sorted_vec::SortedVec;
    ///
    /// let arr = SortedVec::from_sorted_slice(&[1, 2, 3, 2]);
    /// ```
    pub fn from_sorted_slice(slice: &[T]) -> Self {
        debug_assert!(slice.windows(2).all(|w| w[0] <= w[1]));
        Self {
            arr: slice.to_vec(),
        }
    }

    /// Reduce multiple occurrences of a value to a single one
    ///
    /// Complexity: $\mathcal{O}(N)$ comparisons and copies where
    /// - $N$ is the length of the sorted vector
    ///
    /// # Example
    ///
    /// ```
    /// use cp_library::ds::sorted_vec::SortedVec;
    ///
    /// let mut arr = SortedVec::from(vec![1, 2, 2, 3, 4, 4]);
    /// arr.make_unique();
    ///
    /// assert_eq!(*arr, vec![1, 2, 3, 4]);
    /// ```
    pub fn make_unique(&mut self) {
        let mut i = 0;
        for j in 0..self.len() {
            if j + 1 == self.len() || self[j] != self[j + 1] {
                self[i] = self[j].clone();
                i += 1;
            }
        }

        self.resize_with(i, || unreachable!());
    }

    /// Finds the index of the first element which is not smaller than `v`,
    /// or the length of the array if none exists.
    ///
    /// Complexity: $\mathcal{O}(\log N)$ comparisons where
    /// - $N$ is the length of the sorted vector
    ///
    /// # Example
    ///
    /// ```
    /// use cp_library::ds::sorted_vec::SortedVec;
    ///
    /// let arr = SortedVec::from(vec![1, 2, 2, 3, 4, 4]);
    ///
    /// assert_eq!(arr.lower_bound(&4), 4);
    /// assert_eq!(arr.lower_bound(&5), 6);
    /// ```
    pub fn lower_bound(&self, v: &T) -> usize {
        first_true(0, self.len(), |idx| &self[idx] >= v)
    }

    /// Finds the index of the first element which is greater than `v`,
    /// or the length of the array if none exists.
    ///
    /// Complexity: $\mathcal{O}(\log N)$ comparisons where
    /// - $N$ is the length of the sorted vector
    ///
    /// # Example
    ///
    /// ```
    /// use cp_library::ds::sorted_vec::SortedVec;
    ///
    /// let arr = SortedVec::from(vec![1, 2, 2, 3, 4, 4]);
    ///
    /// assert_eq!(arr.upper_bound(&2), 3);
    /// assert_eq!(arr.upper_bound(&5), 6);
    /// ```
    pub fn upper_bound(&self, v: &T) -> usize {
        first_true(0, self.len(), |idx| &self[idx] > v)
    }
}

/// Conversion from a `Vec` by sorting its content
///
/// Complexity: $\mathcal{O}(N \log N)$ comparisons where:
/// - $N$ is the length of the `Vec`
///
/// # Example
///
/// ```
/// use cp_library::ds::sorted_vec::SortedVec;
///
/// let arr = SortedVec::from(vec![2, 3, 1]);
/// assert_eq!(*arr, vec![1, 2, 3]);
/// ```
impl<T> From<Vec<T>> for SortedVec<T>
where
    T: Ord,
{
    fn from(mut arr: Vec<T>) -> Self {
        arr.sort_unstable();
        Self { arr }
    }
}

/// Immutably accesses the internal `Vec` to provide its methods
///
/// # Example
///
/// ```
/// use cp_library::ds::sorted_vec::SortedVec;
///
/// let arr = SortedVec::from_sorted_slice(&vec![1, 2, 3]);
///
/// assert_eq!(arr[2], 3);
/// ```
impl<T> Deref for SortedVec<T>
where
    T: Ord,
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.arr
    }
}

/// Mutably accesses the internal `Vec` to provide its methods
///
/// # Warning
///
/// No checks are made if the sorted vector is accessed through mutable deferencing.
/// It is up to you to keep the vector sorted if you modify it through `Vec` methods!
impl<T> DerefMut for SortedVec<T>
where
    T: Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.arr
    }
}
