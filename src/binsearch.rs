/// Denotes types over which a binary search can be performed.
///
/// This trait is already implemented for all numeric types.
pub trait BinarySearchable: Clone {
    /// Should return the midpoint of two values of `Self`.
    fn midpoint(&self, other: &Self) -> Self;

    /// Should return whether two values of `Self` can be considered close enough
    /// to end the binary search.
    ///
    /// Default implementation gives:
    /// - [`true`] only for unique values for integral types
    /// - [`true`] only for ranges smaller than $10^{-6}$ for floating point types.
    fn close_enough(&self, other: &Self) -> bool;
}

macro_rules! impl_binary_search_integral {
    ($type:ty) => {
        impl BinarySearchable for $type {
            fn midpoint(&self, other: &Self) -> Self {
                (other + self) / 2
            }
            fn close_enough(&self, other: &Self) -> bool {
                other - self <= 1
            }
        }
    };
}

macro_rules! impl_binary_search_floating {
    ($type:ty) => {
        impl BinarySearchable for $type {
            fn midpoint(&self, other: &Self) -> Self {
                (other + self) / 2.
            }
            fn close_enough(&self, other: &Self) -> bool {
                other - self <= 1e-6
            }
        }
    };
}

impl_binary_search_integral!(u8);
impl_binary_search_integral!(u16);
impl_binary_search_integral!(u32);
impl_binary_search_integral!(u64);
impl_binary_search_integral!(u128);
impl_binary_search_integral!(usize);

impl_binary_search_integral!(i8);
impl_binary_search_integral!(i16);
impl_binary_search_integral!(i32);
impl_binary_search_integral!(i64);
impl_binary_search_integral!(i128);
impl_binary_search_integral!(isize);

impl_binary_search_floating!(f32);
impl_binary_search_floating!(f64);

/// Returns the first value in $[l, r)$ for which `predicate` returns [`true`],
/// or $r$ if none exists.
///
/// Conditions: there must exist a value $x_0$ such that `predicate(x)` return [`true`] if
/// and only if $x \ge x_0$.
///
/// Complexity: $\mathcal{O}(\log (r - l))$ calls to `predicate`.
///
/// # Examples
///
/// ```
/// use cp_library::binsearch::first_true;
///
/// let check = |val| val > 6;
/// let fst = first_true(-5, 15, check);
///
/// assert_eq!(fst, 7);
/// ```
///
/// ```
/// use cp_library::binsearch::first_true;
///
/// let check = |val| val > 6;
/// let fst = first_true(-5, 5, check);
///
/// assert_eq!(fst, 5);
/// ```
pub fn first_true<F, N>(l: N, r: N, predicate: F) -> N
where
    N: BinarySearchable,
    F: Fn(N) -> bool,
{
    if predicate(l.clone()) {
        return l;
    }

    let (mut l, mut r) = (l, r);
    while !l.close_enough(&r) {
        let m = l.midpoint(&r);
        if predicate(m.clone()) {
            r = m;
        } else {
            l = m;
        }
    }

    r
}

/// Returns the first value in $[l, r)$ for which `predicate` returns [`Some`]`(y)` and [`Some`]`(y)`,
/// or $r$ and [`None`] if none exists.
///
/// This is useful when an explicit construction is needed.
///
/// Conditions: there must exist a value $x_0$ such that `predicate(x)` return [`Some`] if
/// and only if $x \ge x_0$.
///
/// Complexity: $\mathcal{O}(\log (r - l))$ calls to `predicate`.
///
/// # Examples
///
/// ```
/// use cp_library::binsearch::first_some;
///
/// let check = |val| if val * val > 6 { Some(val * val) } else { None };
/// let (fst, proof) = first_some(0, 15, check);
///
/// assert_eq!(fst, 3);
/// assert_eq!(proof, Some(9));
/// ```
///
/// ```
/// use cp_library::binsearch::first_some;
///
/// let check = |val| if val * val > 6 { Some(val * val) } else { None };
/// let (fst, proof) = first_some(0, 2, check);
///
/// assert_eq!(fst, 2);
/// assert_eq!(proof, None);
/// ```
pub fn first_some<F, N, T>(l: N, r: N, predicate: F) -> (N, Option<T>)
where
    N: BinarySearchable,
    F: Fn(N) -> Option<T>,
{
    if let Some(x) = predicate(l.clone()) {
        return (l, Some(x));
    }

    let (mut l, mut r) = (l, r);
    let mut ans = None;
    while !l.close_enough(&r) {
        let m = l.midpoint(&r);
        if let Some(v) = predicate(m.clone()) {
            r = m;
            ans = Some(v);
        } else {
            l = m;
        }
    }

    (r, ans)
}

/// Returns the first value in $[l, r)$ for which `predicate` returns [`None`],
/// or $r$ if none exists.
///
/// Furthermore, it returns `predicate(x)` for the greatest $x$ for which it is not `None`,
/// or [`None`] if no such $x$ exists.
///
/// This is useful when an explicit construction is needed.
///
/// Conditions: there must exist a value $x_0$ such that `predicate(x)` return [`Some`] if
/// and only if $x \ge x_0$.
///
/// Complexity: $\mathcal{O}(\log (r - l))$ calls to `predicate`.
///
/// # Examples
///
/// ```
/// use cp_library::binsearch::first_none;
///
/// let check = |val| if val * val < 6 { Some(val * val) } else { None };
/// let (fst, proof) = first_none(0, 15, check);
///
/// assert_eq!(fst, 3);
/// assert_eq!(proof, Some(4));
/// ```
///
/// ```
/// use cp_library::binsearch::first_none;
///
/// let check = |val| if val * val < 6 { Some(val * val) } else { None };
/// let (fst, proof) = first_none(5, 15, check);
///
/// assert_eq!(fst, 5);
/// assert_eq!(proof, None);
/// ```
pub fn first_none<F, N, T>(l: N, r: N, predicate: F) -> (N, Option<T>)
where
    N: BinarySearchable,
    F: Fn(N) -> Option<T>,
{
    let mut ans = match predicate(l.clone()) {
        None => return (l, None),
        x => x,
    };

    let (mut l, mut r) = (l, r);
    while !l.close_enough(&r) {
        let m = l.midpoint(&r);
        if let Some(v) = predicate(m.clone()) {
            l = m;
            ans = Some(v);
        } else {
            r = m;
        }
    }

    (r, ans)
}
