use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Index,
    IndexMut,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
};
use crate::maths::traits::numeric::Numeric;
use std::iter::{ Iterator, IntoIterator };

/// A generic tuple.
pub trait Tuple<T: Numeric, Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Add<T, Output = Output>
    + AddAssign<Rhs>
    + AddAssign<T>
    + Div<Rhs, Output = Output>
    + Div<T, Output = Output>
    + DivAssign<Rhs>
    + DivAssign<T>
    + Index<usize>
    + IndexMut<usize>
    + IntoIterator
    + Mul<Rhs, Output = Output>
    + Mul<T, Output = Output>
    + MulAssign<Rhs>
    + MulAssign<T>
    + PartialEq
    + Sub<Rhs, Output = Output>
    + Sub<T, Output = Output>
    + SubAssign<Rhs>
    + SubAssign<T>

{
    /// Returns a tuple where each of the components are of their respective absolute
    /// values.
    fn abs(&self) -> Self;
    /// Returns a tuple where each of the components are rounded up.
    fn ceil(&self) -> Self;
    /// Returns a tuple where each of the components are rounded down.
    fn floor(&self) -> Self;
    /// Returns the linear interpolation given between two tuples, provided `t` 
    /// (which is a ratio). See: https://en.wikipedia.org/wiki/Linear_interpolation
    fn lerp(t0: &Self, t1: &Self, t: T) -> T;
    /// Returns the number of elements in the tuple.
    fn ndim() -> usize;
}

pub mod tuple2 {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct Tuple2<T: Numeric> {
        pub x: T,
        pub y: T,
    }

    impl<T: Numeric> Tuple2<T> {
        pub fn new(x: T, y: T) -> Self {
            Tuple2 {
                x,
                y,
            }
        }
    }

    impl<T: Numeric> Tuple<T> for Tuple2<T> {
        fn ndim() -> usize {
            2
        }

        fn abs(&self) -> Self {
            Tuple2 {
                x: T::abs(self.x),
                y: T::abs(self.y),
            }
        }

        fn ceil(&self) -> Self {
            Tuple2 {
                x: T::ceil(self.x),
                y: T::ceil(self.y),
            }
        }

        fn floor(&self) -> Self {
            Tuple2 {
                x: T::floor(self.x),
                y: T::floor(self.y),
            }
        }

        fn lerp(t0: &Self, t1: &Self, t: T) -> T {
            todo!()
        }
    }

    pub struct Tuple2Iter<T: Numeric> {
        tup: Tuple2<T>,
        index: usize,
    }

    impl<T: Numeric> Iterator for Tuple2Iter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            let result = match self.index {
                0 => self.tup.x,
                1 => self.tup.y,
                _ => return None,
            };
            self.index += 1;
            Some(result)
        }
    }

    impl<T: Numeric> IntoIterator for Tuple2<T> {
        type Item = T;
        type IntoIter = Tuple2Iter<T>;

        fn into_iter(self) -> Self::IntoIter {
            Tuple2Iter {
                tup: self,
                index: 0,
            }
        }
    }

    impl<T: Numeric> PartialEq for Tuple2<T> {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl<T: Numeric> Add<Self> for Tuple2<T> {
        type Output = Tuple2<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Tuple2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<T: Numeric> Add<T> for Tuple2<T> {
        type Output = Tuple2::<T>;

        fn add(self, rhs: T) -> Self::Output {
            Tuple2 {
                x: self.x + rhs,
                y: self.y + rhs,
            }
        }
    }

    impl<T: Numeric> AddAssign<Self> for Tuple2<T> {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl<T: Numeric> AddAssign<T> for Tuple2<T> {
        fn add_assign(&mut self, rhs: T) {
            self.x += rhs;
            self.y += rhs;
        }
    }

    impl<T: Numeric> Sub<Self> for Tuple2<T> {
        type Output = Tuple2<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            Tuple2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl <T: Numeric> Sub<T> for Tuple2<T> {
        type Output = Tuple2<T>;

        fn sub(self, rhs: T) -> Self::Output {
            Tuple2 {
                x: self.x - rhs,
                y: self.y - rhs,
            }
        }
    }

    impl<T: Numeric> SubAssign<Self> for Tuple2<T> {
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    impl<T: Numeric> SubAssign<T> for Tuple2<T> {
        fn sub_assign(&mut self, rhs: T) {
            self.x -= rhs;
            self.y -= rhs;
        }
    }

    impl<T: Numeric> Mul<Self> for Tuple2<T> {
        type Output = Tuple2<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            Tuple2 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }

    impl<T: Numeric> Mul<T> for Tuple2<T> {
        type Output = Tuple2<T>;

        fn mul(self, rhs: T) -> Self::Output {
            Tuple2 {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl<T: Numeric> MulAssign<Self> for Tuple2<T> {
        fn mul_assign(&mut self, rhs: Self) {
            self.x *= rhs.x;
            self.y *= rhs.y;
        }
    }

    impl<T: Numeric> MulAssign<T> for Tuple2<T> {
        fn mul_assign(&mut self, rhs: T) {
            self.x *= rhs;
            self.y *= rhs;
        }
    }

    impl<T: Numeric> Div<Self> for Tuple2<T> {
        type Output = Tuple2<T>;

        fn div(self, rhs: Self) -> Self::Output {
            Tuple2 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            }
        }
    }

    impl <T: Numeric> Div<T> for Tuple2<T> {
        type Output = Tuple2<T>;

        fn div(self, rhs: T) -> Self::Output {
            Tuple2 {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }

    impl<T: Numeric> DivAssign<Self> for Tuple2<T> {
        fn div_assign(&mut self, rhs: Self) {
            self.x /= rhs.x;
            self.y /= rhs.y;
        }
    }

    impl<T: Numeric> DivAssign<T> for Tuple2<T> {
        fn div_assign(&mut self, rhs: T) {
            self.x /= rhs;
            self.y /= rhs;
        }
    }

    impl<T: Numeric> Index<usize> for Tuple2<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            match index {
                0 => &self.x,
                1 => &self.y,
                _ => panic!("index out of bounds: tuple length is {}, but the index is {}.", Tuple2::<T>::ndim(), index),
            }
        }
    }

    impl <T: Numeric> IndexMut<usize> for Tuple2<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            match index {
                0 => &mut self.x,
                1 => &mut self.y,
                _ => panic!("index out of bounds: tuple length is {}, but the index is {}.", Tuple2::<T>::ndim(), index),
            }
        }
    }

}

pub mod tuple3 {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct Tuple3<T: Numeric> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T: Numeric> Tuple3<T> {
        pub fn new(x: T, y: T, z: T) -> Self {
            Tuple3 {
                x,
                y,
                z,
            }
        }
    }

    impl<T: Numeric> Tuple<T> for Tuple3<T> {
        fn ndim() -> usize {
            3
        }

        fn abs(&self) -> Self {
            Tuple3 {
                x: T::abs(self.x),
                y: T::abs(self.y),
                z: T::abs(self.z),
            }
        }

        fn ceil(&self) -> Self {
            Tuple3 {
                x: T::ceil(self.x),
                y: T::ceil(self.y),
                z: T::ceil(self.z),
            }
        }

        fn floor(&self) -> Self {
            Tuple3 {
                x: T::floor(self.x),
                y: T::floor(self.y),
                z: T::floor(self.z),
            }
        }

        fn lerp(t0: &Self, t1: &Self, t: T) -> T {
            todo!()
        }
    }

    pub struct Tuple3Iter<T: Numeric> {
        tup: Tuple3<T>,
        index: usize,
    }

    impl<T: Numeric> Iterator for Tuple3Iter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            let result = match self.index {
                0 => self.tup.x,
                1 => self.tup.y,
                2 => self.tup.z,
                _ => return None,
            };
            self.index += 1;
            Some(result)

        }
    }

    impl<T: Numeric> IntoIterator for Tuple3<T> {
        type Item = T;
        type IntoIter = Tuple3Iter<T>;

        fn into_iter(self) -> Self::IntoIter {
            Tuple3Iter {
                tup: self,
                index: 0,
            }
        }
    }

    impl<T: Numeric> PartialEq for Tuple3<T> {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }
    }

    impl<T: Numeric> Add<Self> for Tuple3<T> {
        type Output = Tuple3<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Tuple3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl<T: Numeric> Add<T> for Tuple3<T> {
        type Output = Tuple3<T>;

        fn add(self, rhs: T) -> Self::Output {
            Tuple3 {
                x: self.x + rhs,
                y: self.y + rhs,
                z: self.z + rhs,
            }
        }
    }

    impl<T: Numeric> AddAssign<Self> for Tuple3<T> {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }

    impl<T: Numeric> AddAssign<T> for Tuple3<T> {
        fn add_assign(&mut self, rhs: T) {
            self.x += rhs;
            self.y += rhs;
            self.z += rhs;
        }
    }

    impl<T: Numeric> Sub<Self> for Tuple3<T> {
        type Output = Tuple3<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            Tuple3 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }

    impl<T: Numeric> Sub<T> for Tuple3<T> {
        type Output = Tuple3<T>;

        fn sub(self, rhs: T) -> Self::Output {
            Tuple3 {
                x: self.x - rhs,
                y: self.y - rhs,
                z: self.z - rhs,
            }
        }
    }

    impl<T: Numeric> SubAssign<Self> for Tuple3<T> {
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
            self.z -= rhs.z;
        }
    }

    impl<T: Numeric> SubAssign<T> for Tuple3<T> {
        fn sub_assign(&mut self, rhs: T) {
            self.x -= rhs;
            self.y -= rhs;
            self.z -= rhs;
        }
    }

    impl<T: Numeric> Mul<Self> for Tuple3<T> {
        type Output = Tuple3<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            Tuple3 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
            }
        }
    }

    impl<T: Numeric> Mul<T> for Tuple3<T> {
        type Output = Tuple3<T>;

        fn mul(self, rhs: T) -> Self::Output {
            Tuple3 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

    impl<T: Numeric> MulAssign<Self> for Tuple3<T> {
        fn mul_assign(&mut self, rhs: Self) {
            self.x *= rhs.x;
            self.y *= rhs.y;
            self.z *= rhs.z;
        }
    }

    impl<T: Numeric> MulAssign<T> for Tuple3<T> {
        fn mul_assign(&mut self, rhs: T) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }

    impl<T: Numeric> Div<Self> for Tuple3<T> {
        type Output = Tuple3<T>;

        fn div(self, rhs: Self) -> Self::Output {
            Tuple3 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
                z: self.z / rhs.z,
            }
        }
    }

    impl<T: Numeric> Div<T> for Tuple3<T> {
        type Output = Tuple3<T>;

        fn div(self, rhs: T) -> Self::Output {
            Tuple3 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }

    impl<T: Numeric> DivAssign<Self> for Tuple3<T> {
        fn div_assign(&mut self, rhs: Self) {
            self.x /= rhs.x;
            self.y /= rhs.y;
            self.z /= rhs.z;
        }
    }

    impl<T: Numeric> DivAssign<T> for Tuple3<T> {
        fn div_assign(&mut self, rhs: T) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
        }
    }

    impl<T: Numeric> Index<usize> for Tuple3<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            match index {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                _ => panic!("index out of bounds: tuple length is {}, but the index is {}.", Tuple3::<T>::ndim(), index),
            }
        }
    }

    impl<T: Numeric> IndexMut<usize> for Tuple3<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            match index {
                0 => &mut self.x,
                1 => &mut self.y,
                2 => &mut self.z,
                _ => panic!("index out of bounds: tuple length is {}, but the index is {}.", Tuple3::<T>::ndim(), index),
            }
        }
    }

}

#[cfg(test)]
mod tuple2_tests {
    use super::{ Tuple, tuple2::Tuple2 };
    type Tuple2i32 = Tuple2<i32>;


    #[test]
    fn new() {
        let tup = Tuple2i32::new(42, 16);
        assert!(tup.x == 42);
        assert!(tup.y == 16);
    }

    #[test]
    fn ndim() {
        assert!(Tuple2i32::ndim() == 2);
    }

    #[test]
    fn eq() {
        let tup_a = Tuple2i32::new(10, 20);
        let tup_b = tup_a.clone();
        assert_eq!(tup_a, tup_b);

        let tup_c = Tuple2i32::new(1, 2);
        let tup_d = Tuple2i32::new(3, 4);
        assert_ne!(tup_c, tup_d);
    }

    #[test]
    fn add_tup() {
        let tup_a = Tuple2i32::new(10, 20);
        let tup_b = Tuple2i32::new(30, 40);
        let expected = Tuple2i32::new(40, 60);
        assert_eq!(tup_a + tup_b, expected);
    }

    #[test]
    fn add_t() {
        let tup = Tuple2i32::new(10, 20);
        assert_eq!(tup + 5, Tuple2i32::new(15, 25));
    }

    #[test]
    fn add_assign_tup() {
        let mut tup = Tuple2i32::new(1, 2);
        tup += Tuple2i32::new(3, 4);
        assert_eq!(tup, Tuple2i32::new(4, 6));
    }

    #[test]
    fn add_assign_t() {
        let mut tup = Tuple2i32::new(1, 2);
        tup += 5;
        assert_eq!(tup, Tuple2i32::new(6, 7));
    }

    #[test]
    fn sub_tup() {
        let tup_a = Tuple2i32::new(50, 25);
        let tup_b = Tuple2i32::new(25, 12);
        let expected = Tuple2i32::new(25, 13);
        assert_eq!(tup_a - tup_b, expected);
    }

    #[test]
    fn sub_t() {
        let tup = Tuple2i32::new(10, 20);
        assert_eq!(tup - 5, Tuple2i32::new(5, 15));
    }

    #[test]
    fn sub_assign_tup() {
        let mut tup = Tuple2i32::new(50, 25);
        tup -= Tuple2i32::new(25, 12);
        assert_eq!(tup, Tuple2i32::new(25, 13));
    }

    #[test]
    fn sub_assign_t() {
        let mut tup = Tuple2i32::new(10, 20);
        tup -= 5;
        assert_eq!(tup, Tuple2i32::new(5, 15));
    }

    #[test]
    fn mul_tup() {
        let tup_a = Tuple2i32::new(1, 2);
        let tup_b = Tuple2i32::new(3, 4);
        assert_eq!(tup_a * tup_b, Tuple2i32::new(3, 8));
    }

    #[test]
    fn mul_t() {
        let tup = Tuple2i32::new(1, 2);
        assert_eq!(tup * 5, Tuple2i32::new(5, 10));
    }

    #[test]
    fn mul_assign_tup() {
        let mut tup = Tuple2i32::new(1, 2);
        tup *= Tuple2i32::new(3, 4);
        assert_eq!(tup, Tuple2i32::new(3, 8));
    }

    #[test]
    fn mul_assign_t() {
        let mut tup = Tuple2i32::new(1, 2);
        tup *= 5;
        assert_eq!(tup, Tuple2i32::new(5, 10));
    }

    #[test]
    fn div_tup() {
        let tup_a = Tuple2i32::new(24, 12);
        let tup_b = Tuple2i32::new(2, 4);
        assert_eq!(tup_a / tup_b, Tuple2i32::new(12, 3));
    }

    #[test]
    fn div_t() {
        let tup = Tuple2i32::new(10, 20);
        assert_eq!(tup / 5, Tuple2i32::new(2, 4))
    }

    #[test]
    fn div_assign_tup() {
        let mut tup = Tuple2i32::new(24, 12);
        tup /= Tuple2i32::new(2, 4);
        assert_eq!(tup, Tuple2i32::new(12, 3));
    }

    #[test]
    fn div_assign_t() {
        let mut tup = Tuple2i32::new(24, 12);
        tup /= 2;
        assert_eq!(tup, Tuple2i32::new(12, 6));
    }

    #[test]
    fn index() {
        let mut tup = Tuple2i32::new(1, 2);
        tup[0] = 3;
        tup[1] = 4;
        assert_eq!(tup[0], 3);
        assert_eq!(tup[1], 4);
    }

    #[test]
    #[should_panic]
    fn index_panic() {
        let tup = Tuple2i32::new(1, 2);
        tup[2];
    }

    #[test]
    fn iter() {
        let mut iter = Tuple2i32::new(1, 2).into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn abs() {
        let tup = Tuple2i32::new(32, -84);
        assert_eq!(tup.abs(), Tuple2i32::new(32, 84));
    }

    #[test]
    fn ceil() {
        let tup = Tuple2::<f32>::new(3.14, 15.92);
        assert_eq!(tup.ceil(), Tuple2::<f32>::new(4.0, 16.0));
    }

    #[test]
    fn floor() {
        let tup = Tuple2::<f32>::new(3.14, 15.92);
        assert_eq!(tup.floor(), Tuple2::<f32>::new(3.0, 15.0));
    }
}


#[cfg(test)]
mod tuple3_tests {
    use super::{ Tuple, tuple3::Tuple3 };
    type Tuple3i32 = Tuple3<i32>;

    #[test]
    fn new() {
        let tup = Tuple3i32::new(13, 51, 79);
        assert_eq!(tup.x, 13);
        assert_eq!(tup.y, 51);
        assert_eq!(tup.z, 79);
    }

    #[test]
    fn ndim() {
        assert_eq!(Tuple3i32::ndim(), 3);
    }

    #[test]
    fn eq() {
        let tup = Tuple3i32::new(13, 51, 79);
        assert_eq!(tup, Tuple3i32::new(13, 51, 79));
        assert_ne!(tup, Tuple3i32::new(13, 51, 80))
    }

    #[test]
    fn add_tup() {
        let tup_a = Tuple3i32::new(1, 2, 3);
        let tup_b = Tuple3i32::new(4, 5, 6);
        assert_eq!(tup_a + tup_b, Tuple3i32::new(5, 7, 9));
    }

    #[test]
    fn add_t() {
        let tup = Tuple3i32::new(1, 2, 3);
        assert_eq!(tup + 5, Tuple3i32::new(6, 7, 8));
    }

    #[test]
    fn add_assign_tup() {
        let mut tup = Tuple3i32::new(1, 2, 3);
        tup += Tuple3i32::new(4, 5, 6);
        assert_eq!(tup, Tuple3i32::new(5, 7, 9));
    }

    #[test]
    fn add_assign_t() {
        let mut tup = Tuple3i32::new(1, 2, 3);
        tup += 5;
        assert_eq!(tup, Tuple3i32::new(6, 7, 8));
    }

    #[test]
    fn sub_tup() {
        let tup_a = Tuple3i32::new(10, 20, 30);
        let tup_b = Tuple3i32::new(4, 5, 6);
        let expected = Tuple3i32::new(6, 15, 24);
        assert_eq!(tup_a - tup_b, expected);
    }

    #[test]
    fn sub_t() {
        let tup = Tuple3i32::new(10, 20, 30);
        assert_eq!(tup - 5, Tuple3i32::new(5, 15, 25));
    }

    #[test]
    fn sub_assign_tup() {
        let mut tup = Tuple3i32::new(10, 20, 30);
        tup -= Tuple3i32::new(4, 5, 6);
        assert_eq!(tup, Tuple3i32::new(6, 15, 24));
    }

    #[test]
    fn sub_assign_t() {
        let mut tup = Tuple3i32::new(10, 20, 30);
        tup -= 5;
        assert_eq!(tup, Tuple3i32::new(5, 15, 25));
    }

    #[test]
    fn mul_tup() {
        let tup_a = Tuple3i32::new(1, 2, 3);
        let tup_b = Tuple3i32::new(4, 5, 6);
        assert_eq!(tup_a * tup_b, Tuple3i32::new(4, 10, 18));
    }

    #[test]
    fn mul_t() {
        let tup = Tuple3i32::new(1, 2, 3);
        assert_eq!(tup * 5, Tuple3i32::new(5, 10, 15));
    }

    #[test]
    fn mul_assign_tup() {
        let mut tup = Tuple3i32::new(1, 2, 3);
        tup *= Tuple3i32::new(4, 5, 6);
        assert_eq!(tup, Tuple3i32::new(4, 10, 18));
    }

    #[test]
    fn mul_assign_t() {
        let mut tup = Tuple3i32::new(1, 2, 3);
        tup *= 5;
        assert_eq!(tup, Tuple3i32::new(5, 10, 15));
    }

    #[test]
    fn div_tup() {
        let tup_a = Tuple3i32::new(12, 24, 36);
        let tup_b = Tuple3i32::new(3, 4, 9);
        assert_eq!(tup_a / tup_b, Tuple3i32::new(4, 6, 4))
    }

    #[test]
    fn div_t() {
        let tup = Tuple3i32::new(12, 24, 36);
        assert_eq!(tup / 3, Tuple3i32::new(4, 8, 12));
    }

    #[test]
    fn div_assign_tup() {
        let mut tup = Tuple3i32::new(12, 24, 36);
        tup /= Tuple3i32::new(3, 4, 18);
        assert_eq!(tup, Tuple3i32::new(4, 6, 2));
    }

    #[test]
    fn div_assign_t() {
        let mut tup = Tuple3i32::new(12, 24, 36);
        tup /= 3;
        assert_eq!(tup, Tuple3i32::new(4, 8, 12));
    }

    #[test]
    fn index() {
        let mut tup = Tuple3i32::new(1, 2, 3);
        tup[0] = 4;
        tup[1] = 5;
        tup[2] = 6;
        assert_eq!(tup[0], 4);
        assert_eq!(tup[1], 5);
        assert_eq!(tup[2], 6);
    }

    #[test]
    #[should_panic]
    fn index_panic() {
        let tup = Tuple3i32::new(1, 2, 3);
        tup[3];
    }

    #[test]
    fn iter() {
        let mut iter = Tuple3i32::new(1, 2, 3).into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn abs() {
        let tup = Tuple3i32::new(32, -84, -121);
        assert_eq!(tup.abs(), Tuple3i32::new(32, 84, 121));
    }

    #[test]
    fn ceil() {
        let tup = Tuple3::<f64>::new(12.51, 89.5, 16.0);
        assert_eq!(tup.ceil(), Tuple3::<f64>::new(13.0, 90.0, 16.0));
    }

    #[test]
    fn floor() {
        let tup = Tuple3::<f64>::new(12.51, 89.5, 16.0);
        assert_eq!(tup.floor(), Tuple3::<f64>::new(12.0, 89.0, 16.0));
    }
}
