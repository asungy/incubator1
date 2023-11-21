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

pub trait Natural<Rhs = Self, Output = Self>:
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

impl<T, Rhs, Output> Natural<Rhs, Output> for T where T:
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
    Natural<Rhs, Output>
    + Neg<Output = Output>
{
    fn abs(self) -> Self;
}

macro_rules! impl_signed {
    ($t:ty) => {
        impl Signed for $t {
            fn abs(self) -> Self {
                self.abs()
            }
        }
    };
}

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(i128);
impl_signed!(isize);
impl_signed!(f32);
impl_signed!(f64);

pub trait Float<Rhs = Self, Output = Self>:
    Signed<Rhs, Output>
{
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
}

macro_rules! impl_float {
    ($t:ty) => {
        impl Float for $t {
            fn ceil(self) -> Self {
                self.ceil()
            }

            fn floor(self) -> Self {
                self.floor()
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);
