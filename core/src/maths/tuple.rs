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
    /// Returns the component-wise fused multiply-add.
    fn fma(t0: Self, t1: Self, t2: Self) -> Self;
    /// Returns the linear interpolation given between two tuples, provided `t`
    /// (which is a ratio). See: https://en.wikipedia.org/wiki/Linear_interpolation
    fn lerp(t0: Self, t1: Self, t: f64) -> Self;
    // Returns the component-wise maximum.
    fn max(t0: Self, t1: Self) -> Self;
    /// Returns the component-wise minimum.
    fn min(t0: Self, t1: Self) -> Self;
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

            fn fma(t0: Self, t1: Self, t2: Self) -> Self {
                t0 * t1 + t2
            }

            fn lerp(t0: Self, t1: Self, t: f64) -> Self {
                let t = t as $t;
                (1.0 - t) * t0 + t * t1
            }

            fn max(t0: Self, t1: Self) -> Self {
                Self {
                    x: t0.x.max(t1.x),
                    y: t0.y.max(t1.y),
                }
            }

            fn min(t0: Self, t1: Self) -> Self {
                Self {
                    x: t0.x.min(t1.x),
                    y: t0.y.min(t1.y),
                }
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

macro_rules! impl_tuple3 {
    ($s:ty, $t:ty) => {
        impl $s {
            fn new(x: $t, y: $t, z: $t) -> Self {
                Self { x, y, z }
            }
        }

        impl Tuple3 for $s {
            type Output = $t;

            fn x(&self) -> Self::Output {
                self.x
            }

            fn y(&self) -> Self::Output {
                self.y
            }

            fn z(&self) -> Self::Output {
                self.z
            }
        }

        impl Tuple for $s {
            fn abs(self) -> Self {
                Self {
                    x: self.x.abs(),
                    y: self.y.abs(),
                    z: self.z.abs(),
                }
            }

            fn ceil(self) -> Self {
                Self {
                    x: self.x.ceil(),
                    y: self.y.ceil(),
                    z: self.z.ceil(),
                }
            }

            fn floor(self) -> Self {
                Self {
                    x: self.x.floor(),
                    y: self.y.floor(),
                    z: self.z.floor(),
                }
            }

            fn fma(t0: Self, t1: Self, t2: Self) -> Self {
                t0 * t1 + t2
            }

            fn lerp(t0: Self, t1: Self, t: f64) -> Self {
                let t = t as $t;
                (1.0 - t) * t0 + t * t1
            }

            fn max(t0: Self, t1: Self) -> Self {
                Self {
                    x: t0.x.max(t1.x),
                    y: t0.y.max(t1.y),
                    z: t0.z.max(t1.z),
                }
            }

            fn min(t0: Self, t1: Self) -> Self {
                Self {
                    x: t0.x.min(t1.x),
                    y: t0.y.min(t1.y),
                    z: t0.z.min(t1.z),
                }
            }

            fn ndim() -> usize {
                3
            }
        }

        impl Add<Self> for $s {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        impl Add<$s> for $t {
            type Output = $s;

            fn add(self, rhs: $s) -> Self::Output {
                Self::Output {
                    x: self + rhs.x,
                    y: self + rhs.y,
                    z: self + rhs.z,
                }
            }
        }

        impl AddAssign<Self> for $s {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        impl Div<Self> for $s {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x / rhs.x,
                    y: self.y / rhs.y,
                    z: self.z / rhs.z,
                }
            }
        }

        impl DivAssign<Self> for $s {
            fn div_assign(&mut self, rhs: Self) {
                self.x /= rhs.x;
                self.y /= rhs.y;
                self.z /= rhs.z;
            }
        }

        impl Index<usize> for $s {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
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
                    2 => &mut self.z,
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
                    z: self.z * rhs.z,
                }
            }
        }

        impl Mul<$s> for $t {
            type Output = $s;

            fn mul(self, rhs: $s) -> Self::Output {
                Self::Output {
                    x: self * rhs.x,
                    y: self * rhs.y,
                    z: self * rhs.z,
                }
            }
        }

        impl MulAssign<Self> for $s {
            fn mul_assign(&mut self, rhs: Self) {
                self.x *= rhs.x;
                self.y *= rhs.y;
                self.z *= rhs.z;
            }
        }

        impl Neg for $s {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }

        impl PartialEq for $s {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        impl Sub<Self> for $s {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
            }
        }

        impl SubAssign<Self> for $s {
            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }

        impl IntoIterator for $s {
            type Item = $t;
            type IntoIter = Tuple3Iter<$t>;

            fn into_iter(self) -> Self::IntoIter {
                Tuple3Iter {
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

#[allow(missing_debug_implementations)]
pub struct Tuple3Iter<T> {
    tup: Box<dyn Tuple3<Output = T>>,
    index: usize,
}

impl<T> Iterator for Tuple3Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.tup.x(),
            1 => self.tup.y(),
            2 => self.tup.z(),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl_tuple3!(Tuple3f32, f32);

#[derive(Debug, Clone, Copy)]
pub struct Tuple3f64 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl_tuple3!(Tuple3f64, f64);

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
        let expected = Tuple2f32::new(8.0, 11.75);
        assert_eq!(Tuple2f32::lerp(a, b, 0.75), expected);
    }

    #[test]
    fn fma() {
        let a = Tuple2f32::new(1., 2.);
        let b = Tuple2f32::new(2., 3.);
        let c = Tuple2f32::new(4., 5.);
        let expected = Tuple2f32::new(6., 11.);
        assert_eq!(Tuple2f32::fma(a, b, c), expected);
    }
}

#[cfg(test)]
mod tuple3_tests {
    use super::{ Tuple3f64, Tuple };

    #[test]
    fn abs() {
        let tup = Tuple3f64::new(-4.5, -9.5, 10.3);
        let expected = Tuple3f64::new(4.5, 9.5, 10.3);
        assert_eq!(tup.abs(), expected);
    }

    #[test]
    fn ceil() {
        let tup = Tuple3f64::new(3.14, 15.92, 15.0);
        let expected = Tuple3f64::new(4.0, 16.0, 15.0);
        assert_eq!(tup.ceil(), expected);
    }

    #[test]
    fn floor() {
        let tup = Tuple3f64::new(3.14, 15.92, 15.0);
        let expected = Tuple3f64::new(3.0, 15.0, 15.0);
        assert_eq!(tup.floor(), expected);
    }

    #[test]
    fn lerp() {
        let a = Tuple3f64::new(2.0, 5.0, 7.0);
        let b = Tuple3f64::new(10.0, 14.0, 6.0);
        assert_eq!(Tuple3f64::lerp(a, b, 0.75), Tuple3f64::new(8.0, 11.75, 6.25));
    }

    #[test]
    fn fma() {
        let a = Tuple3f64::new(1., 2., 3.);
        let b = Tuple3f64::new(4., 5., 6.);
        let c = Tuple3f64::new(7., 8., 9.);
        let expected = Tuple3f64::new(11., 18., 27.);
        assert_eq!(Tuple3f64::fma(a, b, c), expected);
    }
}
