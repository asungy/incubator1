use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
};
use super::abs::Abs;

pub trait Numeric<Rhs = Self, Output = Self>:
    Abs<Item = Self>
    + Add<Rhs, Output = Output>
    + AddAssign<Rhs>
    + Copy
    + Div<Rhs, Output = Output>
    + DivAssign<Rhs>
    + Eq
    + Mul<Rhs, Output = Output>
    + MulAssign<Rhs>
    + PartialEq
    + Sub<Rhs, Output = Output>
    + SubAssign<Rhs>
{}

impl<T, Rhs, Output> Numeric<Rhs, Output> for T where T:
    Abs<Item = Self>
    + Add<Rhs, Output = Output>
    + AddAssign<Rhs>
    + Copy
    + Div<Rhs, Output = Output>
    + DivAssign<Rhs>
    + Eq
    + Mul<Rhs, Output = Output>
    + MulAssign<Rhs>
    + PartialEq
    + Sub<Rhs, Output = Output>
    + SubAssign<Rhs>
{}

impl Abs for u8 {
    type Item = u8;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for u16 {
    type Item = u16;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for u32 {
    type Item = u32;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for u64 {
    type Item = u64;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for u128{
    type Item = u128;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for usize {
    type Item = usize;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for i8 {
    type Item = i8;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for i16 {
    type Item = i16;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for i32 {
    type Item = i32;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for i64 {
    type Item = i64;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for i128 {
    type Item = i128;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for isize {
    type Item = isize;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}
