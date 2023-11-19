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
use super::{
    abs::Abs,
    ceil::Ceil,
};

pub trait Numeric<Rhs = Self, Output = Self>:
    Abs<Item = Self>
    + Add<Rhs, Output = Output>
    + AddAssign<Rhs>
    + Ceil<Item = Self>
    + Copy
    + Div<Rhs, Output = Output>
    + DivAssign<Rhs>
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
    + Ceil<Item = Self>
    + Copy
    + Div<Rhs, Output = Output>
    + DivAssign<Rhs>
    + Mul<Rhs, Output = Output>
    + MulAssign<Rhs>
    + PartialEq
    + Sub<Rhs, Output = Output>
    + SubAssign<Rhs>
{}
