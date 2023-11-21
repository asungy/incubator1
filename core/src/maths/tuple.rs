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
    fn lerp(t0: Self, t1: Self, t: f64) -> Self;
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
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    fn lerp(t0: Self, t1: Self, t: f64) -> Self {
        let t = t as f32;
        (1.0 - t) * t0 + t * t1
    }

    fn ndim() -> usize {
        2
    }
}

impl Add<Self> for Tuple2f32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Tuple2f32> for f32 {
    type Output = Tuple2f32;

    fn add(self, rhs: Tuple2f32) -> Self::Output {
        Self::Output {
            x: self + rhs.x,
            y: self + rhs.y,
        }
    }
}

impl AddAssign<Self> for Tuple2f32 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Div<Self> for Tuple2f32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign<Self> for Tuple2f32 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Index<usize> for Tuple2f32 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!(
                "index out of bounds: tuple length is {}, but the index is {}.",
                Tuple2f32::ndim(),
                index
            ),
        }
    }
}

impl IndexMut<usize> for Tuple2f32 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!(
                "index out of bounds: tuple length is {}, but the index is {}.",
                Tuple2f32::ndim(),
                index
            ),
        }
    }
}

impl Mul<Self> for Tuple2f32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<Tuple2f32> for f32 {
    type Output = Tuple2f32;

    fn mul(self, rhs: Tuple2f32) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl MulAssign<Self> for Tuple2f32 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Neg for Tuple2f32 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq for Tuple2f32 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub<Self> for Tuple2f32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Self> for Tuple2f32 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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

#[allow(missing_debug_implementations)]
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
    use super::{ Tuple2f32, Tuple };

    #[test]
    fn abs() {
        let tup = Tuple2f32::new(-4.5, -9.5);
        let expected = Tuple2f32::new(4.5, 9.5);
        assert_eq!(tup.abs(), expected);
    }

    #[test]
    fn ceil() {
        let tup = Tuple2f32::new(3.14, 15.92);
        let expected = Tuple2f32::new(4.0, 16.0);
        assert_eq!(tup.ceil(), expected);
    }

    #[test]
    fn floor() {
        let tup = Tuple2f32::new(3.14, 15.92);
        let expected = Tuple2f32::new(3.0, 15.0);
        assert_eq!(tup.floor(), expected);
    }

    #[test]
    fn lerp() {
        let a = Tuple2f32::new(2.0, 5.0);
        let b = Tuple2f32::new(10.0, 14.0);
        assert_eq!(Tuple2f32::lerp(a, b, 0.75), Tuple2f32::new(8.0, 11.75));
    }
}
