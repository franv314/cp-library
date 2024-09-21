use std::ops::Add;

/// Basic segment tree over a [monoid](https://en.wikipedia.org/wiki/Monoid) `T`
///
/// `T` must be [`Clone`], furthermore
/// it must implement monoid operations by implementing:
///
/// - [`Default`]: which must provide the identity element.
/// - [`Add`] which must provide the monoid operation.

#[derive(Debug)]
pub struct SegTree<T> {
    arr: Vec<T>,
    size: usize,
}

impl<T> SegTree<T>
    where T: Default + Clone + Add<Output = T>
{
    /// Builds a segment tree of given `size`, filled with identity elements
    ///
    /// Complexity: $\mathcal{O}(N)$ where:
    /// - $N$ is the size of the segment tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::segtree::SegTree;
    ///
    /// let x: SegTree<i32> = SegTree::new(10);
    /// ```
    pub fn new(size: usize) -> Self {
        SegTree { arr: vec![Default::default(); 2 * size], size }
    }

    /// Builds a segment tree from a slice of `T`.
    ///
    /// Complexity: $\mathcal{O}(N)$ additions where:
    /// - $N$ is the size of the segment tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::segtree::SegTree;
    ///
    /// let x = SegTree::from(&[1, 2, 3, 4]);
    /// ```
    pub fn from(array: &[T]) -> Self {
        let size = array.len();
        let mut arr = vec![Default::default(); 2 * size];

        for (i, val) in array.iter().enumerate() {
            arr[i + size] = val.clone();
        }

        for i in (1..size).rev() {
            arr[i] = arr[2 * i].clone() + arr[2 * i + 1].clone();
        }

        SegTree { arr, size }
    }

    /// Perform a range query on the range $[l, r)$.
    ///
    /// Complexity: $\mathcal{O}(\log N)$ additions where:
    /// - $N$ is the size of the segment tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::segtree::SegTree;
    ///
    /// let x = SegTree::from(&[1, 2, 3, 4]);
    /// assert_eq!(x.query(1, 3), 5);
    /// ```
    ///
    /// # Panics
    ///
    /// Only in debug builds, if `l` and `r` do not specify a valid range:
    /// ```should_panic
    /// use cp_library::segtree::SegTree;
    ///
    /// let x = SegTree::from(&[1, 2, 3, 4]);
    /// let y = x.query(3, 2);
    /// ```
    pub fn query(&self, l: usize, r: usize) -> T {
        debug_assert!(l <= r && r <= self.size);

        let mut ans_l: T = Default::default();
        let mut ans_r: T = Default::default();

        let (mut l, mut r) = (l + self.size, r + self.size);
        while l < r {
            if (l & 1) == 1 {
                ans_l = ans_l + self.arr[l].clone();
                l += 1;
            }
            if (r & 1) == 1 {
                r -= 1;
                ans_r = self.arr[r].clone() + ans_r;
            }
        
            (l, r) = (l >> 1, r >> 1);
        }

        ans_l + ans_r
    }

    /// Updates the element at `pos` with value `val`
    ///
    /// Complexity: $\mathcal{O}(\log N)$ additions where:
    /// - $N$ is the size of the segment tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::segtree::SegTree;
    ///
    /// let mut x = SegTree::from(&[1, 2, 3, 4]);
    /// x.update(2, &4);
    /// assert_eq!(x.query(1, 3), 6);
    /// ```
    ///
    /// # Panics
    ///
    /// Only in debug mode, if `pos` is not a valid index.
    ///
    /// ```should_panic
    /// use cp_library::segtree::SegTree;
    ///
    /// let mut x = SegTree::from(&[1, 2, 3, 4]);
    /// x.update(4, &4);
    /// ```
    ///
    /// # Caveat
    ///
    /// The `SegTree` should be declared as `mut` for this method to be used
    ///
    /// ```compile_fail
    /// use cp_library::segtree::SegTree;
    ///
    /// let x = SegTree::from(&[1, 2, 3, 4]);
    /// x.update(2, &4);
    /// ```
    pub fn update(&mut self, pos: usize, val: &T) {
        debug_assert!(pos < self.size);

        let mut pos = pos + self.size;
        self.arr[pos] = val.clone();

        while {
            pos >>= 1;
            pos > 0
        } {
            self.arr[pos] = self.arr[2 * pos].clone() + self.arr[2 * pos + 1].clone();
        }
    }
}
