use std::cmp;

/// Calculate [Z-function](https://cp-algorithms.com/string/z-function.html)
/// of a given slice of an [`Eq`] type `T`.
///
/// Complexity: $\mathcal{O}(N)$ comparisons where:
/// - $N$ is the size of the slice.
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

/// Cyclic dictionary basic factor of a string over an alphabet `T`.
///
/// `T` must be [`Ord`] to ensure comparability.
#[derive(Clone, Debug)]
pub struct DBF {
    n: usize,
    dbf: Vec<Vec<usize>>,
}

impl DBF {
    /// Constructs a DBF from a slice of `T`
    ///
    /// Complexity: $\mathcal{O}(N\log N)$ where
    /// - $N$ is the size of the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::string::DBF;
    /// let dbf = DBF::from("banana".as_bytes());
    /// ```
    pub fn from<T: Ord>(arr: &[T]) -> Self {
        let n = arr.len();

        let mut order = (0..n).collect::<Vec<_>>();
        order.sort_unstable_by_key(|i| &arr[*i]);

        let mut row = vec![0; n];
        row[order[0]] = 0;

        for i in 1..n {
            row[order[i]] = row[order[i - 1]] + ((arr[order[i]] != arr[order[i - 1]]) as usize);
        }

        let mut freq = vec![0; n];
        let mut dbf = vec![row.clone()];

        while {
            let last_row = &dbf[dbf.len() - 1];

            for i in 0..n {
                freq[i] = 0;
            }
            for v in last_row {
                freq[*v] += 1;
            }
            let mut acc = 0;
            for i in 0..n {
                acc += freq[i];
                freq[i] = acc - freq[i];
            }

            let off = 1 << (dbf.len() - 1);
            for i in 0..n {
                let elem = last_row[(i + off) % n];
                row[freq[elem]] = i;
                freq[elem] += 1;
            }

            for i in 0..n {
                freq[i] = 0;
            }
            for v in last_row {
                freq[*v] += 1;
            }
            let mut acc = 0;
            for i in 0..n {
                acc += freq[i];
                freq[i] = acc - freq[i];
            }

            for i in &row {
                let elem = last_row[*i];
                order[freq[elem]] = *i;
                freq[elem] += 1;
            }

            row[order[0]] = 0;

            for i in 1..n {
                let fst_pair = (last_row[order[i]], last_row[(order[i] + off) % n]);
                let snd_pair = (last_row[order[i - 1]], last_row[(order[i - 1] + off) % n]);

                row[order[i]] = row[order[i - 1]] + ((fst_pair != snd_pair) as usize);
            }

            dbf.push(row.clone());

            (1 << (dbf.len() - 1)) < n
        } {}

        Self { n, dbf }
    }

    /// Returns the last row of the DBF (order of cyclic shifts)
    ///
    /// Complexity; $\mathcal{O}(N)$ where:
    /// - $N$ is the size of DBF
    ///
    /// The complexity is due to the copy of the last row
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::string::DBF;
    /// let dbf = DBF::from("banana".as_bytes());
    /// let last_row = dbf.last_row();
    ///
    /// assert_eq!(last_row, vec![3, 2, 5, 1, 4, 0]);
    /// ```
    pub fn last_row(&self) -> Vec<usize> {
        self.dbf[self.dbf.len() - 1].clone()
    }

    /// Gives a representative of a substring $[l, r)$ of the slice.
    ///
    /// Complexity: $\mathcal{O}(1)$
    ///
    /// Returns a pair of [`usize`] representing the substring [l, r).
    /// Substrings of the same length can be compared by comparing
    /// their representatives.
    ///
    /// # Example
    ///
    /// ```
    /// use cp_library::string::DBF;
    /// let dbf = DBF::from("banana".as_bytes());
    ///
    /// assert_eq!(dbf.get_repr(1, 4), (1, 3));
    /// ```
    ///
    /// # Panics
    ///
    /// Only in debug builds, if [l, r) does not specify a valid range
    /// ```should_panic
    /// use cp_library::string::DBF;
    /// let dbf = DBF::from("banana".as_bytes());
    /// let repr = dbf.get_repr(5, 8);
    /// ```
    pub fn get_repr(&self, l: usize, r: usize) -> (usize, usize) {
        debug_assert!(l < r && r <= self.n);

        let h = 63 - (r - l).leading_zeros();
        (
            self.dbf[h as usize][l],
            self.dbf[h as usize][(r - (1 << h)) % self.n],
        )
    }

    /// Compares two substrings $[l_1, r_1)$ and $[l_2, r_2)$
    ///
    /// Complexity: $\mathcal{O}(1)$
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use cp_library::string::DBF;
    /// let dbf = DBF::from("banana".as_bytes());
    ///
    /// assert_eq!(dbf.cmp(1, 4, 0, 2), Ordering::Less)
    /// ```
    ///
    /// # Panics
    ///
    /// Only in debug builds, if either [l1, r1) or [l2, r2) do not specify valid range
    ///
    /// ```should_panic
    /// use cp_library::string::DBF;
    /// let dbf = DBF::from("banana".as_bytes());
    /// let ord = dbf.cmp(3, 1, 0, 3);
    /// ```
    pub fn cmp(&self, l1: usize, r1: usize, l2: usize, r2: usize) -> cmp::Ordering {
        debug_assert!(l1 < r1 && r1 <= self.n);
        debug_assert!(l2 < r2 && r2 <= self.n);

        let common_length = cmp::min(r1 - l1, r2 - l2);
        let repr1 = self.get_repr(l1, l1 + common_length);
        let repr2 = self.get_repr(l2, l2 + common_length);

        match repr1.cmp(&repr2) {
            cmp::Ordering::Equal => (r1 - l1).cmp(&(r2 - l2)),
            order => order,
        }
    }
}

/// Constructs the [Suffix array](https://cp-algorithms.com/string/suffix-array.html) of a slice of `T`
///
/// `T` must be [`Ord`]
///
/// Complexity: $\mathcal{O}(N\log N)$ comparisons where:
/// - $N$ is the size of the slice.
///
/// # Examples
///
/// ```
/// use cp_library::string::suffix_array;
/// let suff_arr = suffix_array("banana".as_bytes());
///
/// assert_eq!(suff_arr, [5, 3, 1, 0, 4, 2]);
/// ```
pub fn suffix_array<T: Ord>(arr: &[T]) -> Vec<usize> {
    let n = arr.len();
    let arr = arr
        .iter()
        .map(Some)
        .chain(std::iter::once(None))
        .collect::<Vec<_>>();
    let dbf = DBF::from(&arr);
    let last_row = dbf.last_row();

    let mut suffix_array = vec![0; n];
    for (i, &x) in last_row.iter().enumerate() {
        if x != 0 {
            suffix_array[x - 1] = i;
        }
    }

    suffix_array
}

/// Constructs the [Suffix array](https://cp-algorithms.com/string/suffix-array.html)
/// and [LCP](https://cp-algorithms.com/string/suffix-array.html#longest-common-prefix-of-two-substrings-without-additional-memory)
/// array of a slice of `T`
///
/// Complexity: $\mathcal{O}(N\log N)$ comparisons where:
/// - $N$ is the size of the slice.
///
/// # Examples
///
/// ```
/// use cp_library::string::lcp;
/// let (suff_arr, lcp_arr) = lcp("banana".as_bytes());
///
/// assert_eq!(suff_arr, [5, 3, 1, 0, 4, 2]);
/// assert_eq!(lcp_arr, [0, 1, 3, 0, 0, 2]);
/// ```
pub fn lcp<T: Ord>(arr: &[T]) -> (Vec<usize>, Vec<usize>) {
    let n = arr.len();
    let suffix_array = suffix_array(arr);

    let mut inv = vec![0; n];
    for (i, &x) in suffix_array.iter().enumerate() {
        inv[x] = i;
    }

    let mut lcp = vec![0; n];
    let mut k = 0;

    for i in 0..n {
        if inv[i] == n - 1 {
            k = 0;
        } else {
            let j = suffix_array[inv[i] + 1];
            while i + k < n && j + k < n && arr[i + k] == arr[j + k] {
                k += 1;
            }

            lcp[inv[i] + 1] = k;
            k = k.saturating_sub(1);
        }
    }

    (suffix_array, lcp)
}
