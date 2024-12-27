use super::*;

impl<T: Magma> Magma for Option<T> {
    fn op(self, other: Self) -> Self {
        match (self, other) {
            (None, r) => r,
            (l, None) => l,
            (Some(l), Some(r)) => Some(l.op(r)),
        }
    }
}

impl<T: Semigroup> Semigroup for Option<T> {}

impl<T: Semigroup> Monoid for Option<T> {
    const ID: Self = None;
}

macro_rules! impl_monoid_for_num {
    ($type:ty) => {
        impl Magma for $type {
            fn op(self, other: Self) -> Self {
                self + other
            }
        }

        impl Semigroup for $type {}

        impl Monoid for $type {
            const ID: Self = 0 as $type;
        }
    };
}

macro_rules! impl_abelian_for_num {
    ($type:ty) => {
        impl Group for $type {
            fn inv(self) -> Self {
                -self
            }
        }

        impl Abelian for $type {}
    };
}

impl_monoid_for_num!(i8);
impl_monoid_for_num!(i16);
impl_monoid_for_num!(i32);
impl_monoid_for_num!(i64);
impl_monoid_for_num!(i128);
impl_monoid_for_num!(isize);

impl_monoid_for_num!(u8);
impl_monoid_for_num!(u16);
impl_monoid_for_num!(u32);
impl_monoid_for_num!(u64);
impl_monoid_for_num!(u128);
impl_monoid_for_num!(usize);

impl_monoid_for_num!(f32);
impl_monoid_for_num!(f64);

impl_abelian_for_num!(i8);
impl_abelian_for_num!(i16);
impl_abelian_for_num!(i32);
impl_abelian_for_num!(i64);
impl_abelian_for_num!(i128);
impl_abelian_for_num!(isize);

impl_abelian_for_num!(f32);
impl_abelian_for_num!(f64);
