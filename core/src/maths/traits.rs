use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};

pub trait Abs {
    fn abs(self) -> Self;
}

macro_rules! impl_abs {
    ($t:ty) => {
        impl Abs for $t {
            fn abs(self) -> Self {
                self.abs()
            }
        }
    };
}

impl_abs!(i8);
// NOTE: CONTINUE HERE!

pub trait Ceil {
    fn ceil(self) -> Self;
}

pub trait Floor {
    fn floor(self) -> Self;
}

pub trait Numeric<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + AddAssign<Rhs>
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
    Add<Rhs, Output = Output>
    + AddAssign<Rhs>
    + Copy
    + Div<Rhs, Output = Output>
    + DivAssign<Rhs>
    + Mul<Rhs, Output = Output>
    + MulAssign<Rhs>
    + PartialEq
    + Sub<Rhs, Output = Output>
    + SubAssign<Rhs>
{}

pub trait Signed<Rhs = Self, Output = Self>:
    Numeric<Rhs, Output>
    + Abs
    + Neg<Output = Output>
{}

impl<T, Rhs, Output> Signed<Rhs, Output> for T where T:
    Numeric<Rhs, Output>
    + Abs
    + Neg<Output = Output>
{}


pub trait Float<Rhs = Self, Output = Self>:
    Signed<Rhs, Output>
    + Ceil
    + Floor
{}

impl<T, Rhs, Output> Float<Rhs, Output> for T where T:
    Signed<Rhs, Output>
    + Ceil
    + Floor
{}
