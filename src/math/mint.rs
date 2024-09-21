/// Macro to generate modular integer classes (prime moduli)
///
/// The macro accepts three parameters:
/// - `$name`: the name the struct should have
/// - `$mod`: the prime modulus to use
/// - `$type`: the type used for internal operations, should be able to hold $\text{MOD}^2$
///
/// # Examples
///
/// Basic operations are implemented for left and right operands of `gen_mint!` type
/// as well as left operand of `gen_mint!` type and right operand of primitive type.
///
/// Complexity:
/// - $\mathcal{O}(1)$ for addition, subtraction and multiplication;
/// - $\mathcal{O}(\log \text{MOD})$ for division.
///
/// ```
/// use cp_library::gen_mint;
/// gen_mint!(Mint, 37, i64);
///
/// assert_eq!(Mint(25) + Mint(26), Mint(14));
/// assert_eq!(Mint(25) + 26, Mint(14));
///
/// assert_eq!(Mint(5) - Mint(7), Mint(35));
/// assert_eq!(Mint(5) - 7, Mint(35));
///
/// assert_eq!(Mint(6) * Mint(7), Mint(5));
/// assert_eq!(Mint(6) * 7, Mint(5));
///
/// assert_eq!(Mint(3).inv(), Mint(25));
/// assert_eq!(Mint(2) / Mint(3), Mint(13));
/// assert_eq!(Mint(2) / 3, Mint(13));
/// ```
///
/// Exponentiation ($x^y$) is supported.
///
/// Complexity:
/// - $\mathcal{O}(\log y)$ if $y \ge 0$
/// - $\mathcal{O}(\log y + \log \text{MOD})$ if $y < 0$.
///
/// ```
/// use cp_library::gen_mint;
/// gen_mint!(Mint, 37, i64);
///
/// assert_eq!(Mint(13) ^ 0, Mint(1));
/// assert_eq!(Mint(13) ^ 5, Mint(35));
/// assert_eq!(Mint(13) ^ -5, Mint(18));
/// ```
///
/// Operation + assignment is supported, right hand side can be a `gen_mint!` type or a primitive type
///
/// ```
/// use cp_library::gen_mint;
/// gen_mint!(Mint, 37, i64);
///
/// let mut x = Mint(15);
///
/// x += Mint(35);
/// assert_eq!(x, Mint(13));
///
/// x -= Mint(14);
/// assert_eq!(x, Mint(36));
///
/// x *= Mint(4);
/// assert_eq!(x, Mint(33));
///
/// x /= Mint(11);
/// assert_eq!(x, Mint(3));
///
/// x *= 11;
/// assert_eq!(x, Mint(33));
///
/// x /= 4;
/// assert_eq!(x, Mint(36));
///
/// x += 14;
/// assert_eq!(x, Mint(13));
///
/// x -= 35;
/// assert_eq!(x, Mint(15));
///
/// x ^= -4;
/// assert_eq!(x, Mint(33));
/// ```
///
/// The type is `Display`
/// ```
/// use cp_library::gen_mint;
/// use cp_library::inout::OutputWriter;
///
/// gen_mint!(Mint, 37, i64);
///
/// let mut buf = Vec::new();
/// {
///     let mut writer = OutputWriter::new(&mut buf);
///     writer.put(&(Mint(32) + Mint(6)))
/// }
///
/// assert_eq!(buf, b"1".to_vec())
/// ```
///
/// # Panics
///
/// Only in debug mode, if a right operand of primitive type is not in the range $[0, \text{MOD})$.
///
/// ```should_panic
/// use cp_library::gen_mint;
///
/// gen_mint!(Mint, 37, i64);
/// let x = Mint(23) + 47;
/// ```
#[macro_export]
macro_rules! gen_mint {
    ($name: ident, $mod: literal, $type: ty) => {
        use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, SubAssign, DivAssign, BitXor, BitXorAssign};
        use std::fmt::{Display, Result};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct $name($type);

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
                write!(f, "{}", self.0)
            }
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self((self.0 + rhs.0) % $mod)
            }
        }

        impl Add<$type> for $name {
            type Output = Self;
            fn add(self, rhs: $type) -> Self {
                debug_assert!(0 <= rhs && rhs < $mod);

                Self((self.0 + rhs) % $mod)
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
                self.0 %= $mod;
            }
        }

        impl AddAssign<$type> for $name {
            fn add_assign(&mut self, rhs: $type) {
                debug_assert!(0 <= rhs && rhs < $mod);

                self.0 += rhs;
                self.0 %= $mod;
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self((self.0 - rhs.0 + $mod) % $mod)
            }
        }

        impl Sub<$type> for $name {
            type Output = Self;
            fn sub(self, rhs: $type) -> Self {
                debug_assert!(0 <= rhs && rhs < $mod);

                Self((self.0 - rhs + $mod) % $mod)
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 += $mod - rhs.0;
                self.0 %= $mod;
            }
        }

        impl SubAssign<$type> for $name {
            fn sub_assign(&mut self, rhs: $type) {
                debug_assert!(0 <= rhs && rhs < $mod);

                self.0 += $mod - rhs;
                self.0 %= $mod;
            }
        }

        impl Mul for $name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                Self((self.0 * rhs.0) % $mod)
            }
        }

        impl Mul<$type> for $name {
            type Output = Self;
            fn mul(self, rhs: $type) -> Self {
                debug_assert!(0 <= rhs && rhs < $mod);

                Self((self.0 * rhs) % $mod)
            }
        }

        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
                self.0 %= $mod;
            }
        }

        impl MulAssign<$type> for $name {
            fn mul_assign(&mut self, rhs: $type) {
                debug_assert!(0 <= rhs && rhs < $mod);

                self.0 *= rhs;
                self.0 %= $mod;
            }
        }

        impl $name {
            fn pow(&self, exp: i64) -> Self {
                debug_assert!(exp >= 0);

                let mut exp = exp;
                let mut ans = Self(1);
                let mut base = *self;

                while exp > 0 {
                    if exp & 1 == 1 {
                        ans *= base;
                    }
                    base *= base;
                    exp >>= 1;
                }

                ans
            }

            pub fn inv(&self) -> Self { self.pow($mod - 2) }
        }

        impl BitXor<i64> for $name {
            type Output = Self;

            fn bitxor(self, rhs: i64) -> Self {
                if rhs >= 0 {
                    self.pow(rhs)
                } else {
                    self.pow(-rhs).inv()
                }
            }
        }

        impl BitXorAssign<i64> for $name {
            fn bitxor_assign(&mut self, rhs: i64) {
                *self = *self ^ rhs;
            }
        }

        impl Div for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                self * rhs.inv()
            }
        }

        impl Div<$type> for $name {
            type Output = Self;
            fn div(self, rhs: $type) -> Self {
                self * $name(rhs).inv()
            }
        }

        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                self.0 = (self.0 * rhs.inv().0) % $mod;
            }
        }

        impl DivAssign<$type> for $name {
            fn div_assign(&mut self, rhs: $type) {
                self.0 = (self.0 * $name(rhs).inv().0) % $mod;
            }
        }
    }
}
