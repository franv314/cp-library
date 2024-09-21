use std::ops::{Mul, Div, Rem};

mod hidden {
    pub trait IsZero {
        fn is_zero(&self) -> bool;
    }

    macro_rules! impl_is_zero {
        ($type:ty) => {
            impl IsZero for $type {
                fn is_zero(&self) -> bool { *self == 0 as $type }
            }
        };
    }

    impl_is_zero!(u8);
    impl_is_zero!(u16);
    impl_is_zero!(u32);
    impl_is_zero!(u64);
    impl_is_zero!(u128);
    impl_is_zero!(usize);

    impl_is_zero!(i8);
    impl_is_zero!(i16);
    impl_is_zero!(i32);
    impl_is_zero!(i64);
    impl_is_zero!(i128);
    impl_is_zero!(isize);
}

use hidden::IsZero;

/// Computes the greatest common divisor of $a$ and $b$.
///
/// Complexity: $\mathcal{O}(\log \max (a, b))$
///
/// ```
/// use cp_library::math::nt::gcd;
///
/// let g = gcd(120, 42);
/// assert_eq!(g, 6);
/// ```
pub fn gcd<N>(a: N, b: N) -> N
    where N: Copy + IsZero + Rem<Output = N>
{
    if b.is_zero() { a } else { gcd(b, a % b) }
}

/// Computes the greatest common divisor of $a$ and $b$.
///
/// Complexity: $\mathcal{O}(\log \max (a, b))$
///
/// ```
/// use cp_library::math::nt::lcm;
///
/// let g = lcm(120, 42);
/// assert_eq!(g, 840);
/// ```
pub fn lcm<N>(a: N, b: N) -> N
    where N: Copy + IsZero + Mul<Output = N> + Div<Output = N> + Rem<Output = N> + Eq
{
    a / gcd(a, b) * b
}
