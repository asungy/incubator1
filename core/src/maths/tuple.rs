use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Index,
    IndexMut,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};
use std::marker::Copy;
use std::cmp::PartialEq;
use std::iter::{ Iterator, IntoIterator };

pub trait Tuple<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + AddAssign<Rhs>
    + Copy
    + Div<Rhs, Output = Output>
    + DivAssign<Rhs>
    + Index<usize>
    + IndexMut<usize>
    + IntoIterator
    + Mul<Rhs, Output = Output>
    + MulAssign<Rhs>
    + Neg
    + PartialEq
    + Sub<Rhs, Output = Output>
    + SubAssign<Rhs>
{
    /// Returns a tuple where each of the components are of their respective absolute
    /// values.
    fn abs(self) -> Self;
    /// Returns a tuple where each of the components are rounded up.
    fn ceil(self) -> Self;
    /// Returns a tuple where each of the components are rounded down.
    fn floor(self) -> Self;
    /// Returns the linear interpolation given between two tuples, provided `t`
    /// (which is a ratio). See: https://en.wikipedia.org/wiki/Linear_interpolation
    fn lerp(t0: &Self, t1: &Self, t: f64) -> Self;
    /// Returns the number of elements in the tuple.
    fn ndim() -> usize;
}

pub trait Tuple2 {
    type Output;
    fn x(&self) -> Self::Output;
    fn y(&self) -> Self::Output;
}

pub trait Tuple3 {
    type Output;
    fn x(&self) -> Self::Output;
    fn y(&self) -> Self::Output;
    fn z(&self) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple2f32 {
    pub x: f32,
    pub y: f32,
}

impl Tuple2f32 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Tuple2 for Tuple2f32 {
    type Output = f32;

    fn x(&self) -> Self::Output {
        self.x
    }

    fn y(&self) -> Self::Output {
        self.y
    }
}

impl Tuple for Tuple2f32 {
    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    fn ceil(self) -> Self {
        todo!()
    }

    fn floor(self) -> Self {
        todo!()
    }

    fn lerp(t0: &Self, t1: &Self, t: f64) -> Self {
        todo!()
    }

    fn ndim() -> usize {
        todo!()
    }
}

impl Add<Self> for Tuple2f32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Self> for Tuple2f32 {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Div<Self> for Tuple2f32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl DivAssign<Self> for Tuple2f32 {
    fn div_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Index<usize> for Tuple2f32 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl IndexMut<usize> for Tuple2f32 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}

impl Mul<Self> for Tuple2f32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl MulAssign<Self> for Tuple2f32 {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Neg for Tuple2f32 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl PartialEq for Tuple2f32 {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Sub<Self> for Tuple2f32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl SubAssign<Self> for Tuple2f32 {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl IntoIterator for Tuple2f32 {
    type Item = f32;
    type IntoIter = Tuple2Iter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        Tuple2Iter {
            tup: Box::new(self),
            index: 0,
        }
    }
}

pub struct Tuple2Iter<T> {
    tup: Box<dyn Tuple2<Output = T>>,
    index: usize,
}

impl<T> Iterator for Tuple2Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.tup.x(),
            1 => self.tup.y(),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

macro_rules! impl_tuple2 {
    ($i:item, $t:ty) => {

    };
}

#[cfg(test)]
mod tuple2_tests {
    use super::Tuple2f32;

    #[test]
    fn abs() {
        let tup = Tuple2f32::new(-4.5, -9.5);
        let expected = Tuple2f32::new(4.5, 9.5);
        assert_eq!(tup, expected);
    }

    #[test]
    fn ceil() {
        let tup = Tuple2f32::new(3.14, 15.92);
        let expected = Tuple2f32::new(4.0, 16.0);
        assert_eq!(tup, expected);
    }

    #[test]
    fn floor() {
        let tup = Tuple2f32::new(3.14, 15.92);
        let expected = Tuple2f32::new(3.0, 15.0);
        assert_eq!(tup, expected);
    }

    #[test]
    fn lerp() {
        let a = Tuple2f32::new(2.0, 5.0);
        let b = Tuple2f32::new(10.0, 14.0);
    }
}
