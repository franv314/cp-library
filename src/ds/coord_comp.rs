use crate::ds::sorted_vec::SortedVec;

/// A `SortedVec` wrapper able to provide coordinate compression
pub struct CoordinateCompressor<T> {
    coords: SortedVec<T>,
}

impl<T> CoordinateCompressor<T>
where
    T: Ord + Clone,
{
    /// Creates a compressor from a list of coordinates
    ///
    /// Complexity: $\mathcal{O}(N \log N)$ comparisons where
    /// - $N$ is the number of coordinates.
    ///
    /// # Example
    ///
    /// ```
    /// use cp_library::ds::coord_comp::CoordinateCompressor;
    ///
    /// let comp = CoordinateCompressor::from_coords(vec![1, 2, 3]);
    /// ```
    pub fn from_coords(coords: Vec<T>) -> Self {
        let mut coords = SortedVec::from(coords);
        coords.make_unique();
        CoordinateCompressor { coords }
    }

    /// Returns the number of stored coordinates
    ///
    /// Complexity: $\mathcal{O}(1)$
    ///
    /// # Example
    ///
    /// ```
    /// use cp_library::ds::coord_comp::CoordinateCompressor;
    ///
    /// let comp = CoordinateCompressor::from_coords(vec![100, 200, 300]);
    /// assert_eq!(comp.size(), 3);
    /// ```
    pub fn size(&self) -> usize {
        self.coords.len()
    }

    /// Compresses a given coordinate
    ///
    /// Complexity: $\mathcal{O}(\log N)$ comparisons where
    /// - $N$ is the number of coordinates.
    ///
    /// # Example
    ///
    /// ```
    /// use cp_library::ds::coord_comp::CoordinateCompressor;
    ///
    /// let comp = CoordinateCompressor::from_coords(vec![100, 200, 300]);
    /// assert_eq!(comp.compress(&200), 1);
    /// ```
    ///
    /// # Panics
    ///
    /// Only in debug builds, if the given coordinate is not present in the compressor
    ///
    /// ```should_panic
    /// use cp_library::ds::coord_comp::CoordinateCompressor;
    ///
    /// let comp = CoordinateCompressor::from_coords(vec![100, 200, 300]);
    /// let coord = comp.compress(&201);
    /// ```
    pub fn compress(&self, coord: &T) -> usize {
        debug_assert!(self.coords.binary_search(coord).is_ok());
        self.coords.lower_bound(coord)
    }

    /// Decompresses a given coordinate
    ///
    /// Complexity: $\mathcal{O}(1)$
    ///
    /// # Example
    ///
    /// ```
    /// use cp_library::ds::coord_comp::CoordinateCompressor;
    ///
    /// let comp = CoordinateCompressor::from_coords(vec![100, 200, 300]);
    /// assert_eq!(comp.decompress(2), 300);
    /// ```
    ///
    /// # Panics
    ///
    /// Only in debug builds, if the given coordinate is out of bounds
    ///
    /// ```should_panic
    /// use cp_library::ds::coord_comp::CoordinateCompressor;
    ///
    /// let comp = CoordinateCompressor::from_coords(vec![100, 200, 300]);
    /// let coord = comp.decompress(3);
    /// ```
    pub fn decompress(&self, coord: usize) -> T {
        debug_assert!(coord < self.coords.len());
        self.coords[coord].clone()
    }
}
