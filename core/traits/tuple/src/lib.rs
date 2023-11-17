use std::ops::{
    Add,
};

pub trait Numeric<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
{}

impl<T, Rhs, Output> Numeric<Rhs, Output> for T where T:
    Add<Rhs, Output = Output>
{}

pub trait Tuple2<T: Numeric> {
    /// Returns the number of elements in the vector.
    fn ndim() -> usize;
}
