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

macro_rules! impl_tuple2 {
    ($s:ty, $t:ty) => {
        impl $s {
            fn new(x: $t, y: $t) -> Self {
                Self { x, y }
            }
        }

        impl Tuple2 for $s {
            type Output = $t;

            fn x(&self) -> Self::Output {
                self.x
            }

            fn y(&self) -> Self::Output {
                self.y
            }
        }

        impl Tuple for $s {
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
                let t = t as $t;
                (1.0 - t) * t0 + t * t1
            }

            fn ndim() -> usize {
                2
            }
        }

        impl Add<Self> for $s {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Add<$s> for $t {
            type Output = $s;

            fn add(self, rhs: $s) -> Self::Output {
                Self::Output {
                    x: self + rhs.x,
                    y: self + rhs.y,
                }
            }
        }

        impl AddAssign<Self> for $s {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl Div<Self> for $s {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x / rhs.x,
                    y: self.y / rhs.y,
                }
            }
        }

        impl DivAssign<Self> for $s {
            fn div_assign(&mut self, rhs: Self) {
                self.x /= rhs.x;
                self.y /= rhs.y;
            }
        }

        impl Index<usize> for $s {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!(
                        "index out of bounds: tuple length is {}, but the index is {}.",
                        <$s>::ndim(),
                        index
                    ),
                }
            }
        }

        impl IndexMut<usize> for $s {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => panic!(
                        "index out of bounds: tuple length is {}, but the index is {}.",
                        <$s>::ndim(),
                        index
                    ),
                }
            }
        }

        impl Mul<Self> for $s {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                }
            }
        }

        impl Mul<$s> for $t {
            type Output = $s;

            fn mul(self, rhs: $s) -> Self::Output {
                Self::Output {
                    x: self * rhs.x,
                    y: self * rhs.y,
                }
            }
        }

        impl MulAssign<Self> for $s {
            fn mul_assign(&mut self, rhs: Self) {
                self.x *= rhs.x;
                self.y *= rhs.y;
            }
        }

        impl Neg for $s {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        impl PartialEq for $s {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl Sub<Self> for $s {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl SubAssign<Self> for $s {
            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl IntoIterator for $s {
            type Item = $t;
            type IntoIter = Tuple2Iter<$t>;

            fn into_iter(self) -> Self::IntoIter {
                Tuple2Iter {
                    tup: Box::new(self),
                    index: 0,
                }
            }
        }

    };
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

#[derive(Debug, Clone, Copy)]
pub struct Tuple2f32 {
    pub x: f32,
    pub y: f32,
}
impl_tuple2!(Tuple2f32, f32);

#[derive(Debug, Clone, Copy)]
pub struct Tuple2f64 {
    pub x: f64,
    pub y: f64,
}
impl_tuple2!(Tuple2f64, f64);


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
