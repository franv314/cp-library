/// Common implementation of algebraic traits
pub mod instances;

/// A [Magma](https://en.wikipedia.org/wiki/Magma_(algebra)) has a binary operation
pub trait Magma {
    /// The binary operation of the magma
    fn op(self, other: Self) -> Self;
}

/// A [Semigroup](https://en.wikipedia.org/wiki/Semigroup) must satisfy associativity
///
/// This is marker trait, used to indicate that a [`Magma`] is associative
pub trait Semigroup: Magma {}

/// A [Monoid](https://en.wikipedia.org/wiki/Monoid) must have a neutral element.
///
/// This trait is implemented for all builtin numeric types, with addition as the operation.
/// Furthermore, if `T` is [`Semigroup`], then [`Option<T>`] is automatically [`Monoid`] with [`None`] as the neutral element.
pub trait Monoid: Semigroup {
    /// The neutral element of the monoid
    const ID: Self;
}

/// A [Group](https://en.wikipedia.org/wiki/Group_(algebra)) must have inverses for all elements.
pub trait Group: Monoid {
    /// The inverse element on the group.
    fn inv(self) -> Self;
}

/// An [Abelian](https://en.wikipedia.org/wiki/Abelian_group) group must satisfy commutativity
///
/// This is marker trait, used to indicate that a [`Group`] is commutative
///
/// This trait is implemented for all signed builtin numeric types, with addition as the operation
pub trait Abelian: Group {}
