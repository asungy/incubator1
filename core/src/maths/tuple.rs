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
use crate::maths::traits::{ Numeric, Float, Signed };
use std::iter::{ Iterator, IntoIterator };

pub trait BaseTuple<T: Numeric, Rhs = Self, Output = Self>:
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
    /// Returns the number of elements in the tuple.
    fn ndim() -> usize;
}

pub trait SignedTuple<T: Signed, Rhs = Self, Output = Self>:
    BaseTuple<T, Rhs, Output>
    + Neg
{
    /// Returns a tuple where each of the components are of their respective absolute
    /// values.
    fn abs(&self) -> Self;
}

pub trait FloatTuple<T: Float, Rhs = Self, Output = Self>:
    SignedTuple<T, Rhs, Output>
{
    /// Returns the linear interpolation given between two tuples, provided `t` 
    /// (which is a ratio). See: https://en.wikipedia.org/wiki/Linear_interpolation
    fn lerp(t0: &Self, t1: &Self, t: T) -> Self;
    /// Returns a tuple where each of the components are rounded up.
    fn ceil(&self) -> Self;
    /// Returns a tuple where each of the components are rounded down.
    fn floor(&self) -> Self;
}

pub mod tuple2 {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct Tuple2<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Tuple2<T> {
        pub fn new(x: T, y: T) -> Self {
            Tuple2 {
                x,
                y,
            }
        }
    }

    impl<T: Numeric> BaseTuple<T> for Tuple2<T> {
        fn ndim() -> usize {
            2
        }
    }

    impl<T: Signed> SignedTuple<T> for Tuple2<T> {
        fn abs(&self) -> Self {
            Tuple2 {
                x: T::abs(self.x),
                y: T::abs(self.y),
            }
        }
    }

    impl<T: Signed> Neg for Tuple2<T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Tuple2 {
                x: -self.x,
                y: -self.y,
            }
        }
    }

    impl<T: Float> FloatTuple<T> for Tuple2<T> {
        fn lerp(t0: &Self, t1: &Self, t: T) -> Self {
            todo!()
        }

        fn ceil(&self) -> Self {
            Tuple2 {
                x: self.x.ceil(),
                y: self.y.ceil(),
            }
        }

        fn floor(&self) -> Self {
            Tuple2 {
                x: self.x.floor(),
                y: self.y.floor(),
            }
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

    impl<T: Numeric> IndexMut<usize> for Tuple2<T> {
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

    impl<T: Numeric> BaseTuple<T> for Tuple3<T> {
        fn ndim() -> usize {
            3
        }
    }

    impl<T: Signed> SignedTuple<T> for Tuple3<T> {
        fn abs(&self) -> Self {
            Tuple3 {
                x: self.x.abs(),
                y: self.y.abs(),
                z: self.z.abs(),
            }
        }
    }

    impl<T: Signed> Neg for Tuple3<T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Tuple3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }

    impl<T: Float> FloatTuple<T> for Tuple3<T> {
        fn lerp(t0: &Self, t1: &Self, t: T) -> Self {
            todo!()
        }

        fn ceil(&self) -> Self {
            Tuple3 {
                x: self.x.ceil(),
                y: self.y.ceil(),
                z: self.z.ceil(),
            }
        }

        fn floor(&self) -> Self {
            Tuple3 {
                x: self.x.floor(),
                y: self.y.floor(),
                z: self.z.floor(),
            }
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

#[macro_export]
macro_rules! tup {
    ($x:expr, $y:expr) => {
        $crate::maths::tuple::tuple2::Tuple2::new($x, $y)
    };
    ($x:expr, $y:expr; $t:ty) => {
        $crate::maths::tuple::tuple2::Tuple2::<$t>::new($x, $y)
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::maths::tuple::tuple3::Tuple3::new($x, $y, $z)
    };
    ($x:expr, $y:expr, $z:expr; $t:ty) => {
        $crate::maths::tuple::tuple3::Tuple3::<$t>::new($x, $y, $z)
    };
}

#[cfg(test)]
mod tuple2_tests {
    use super::{
        BaseTuple,
        FloatTuple,
        SignedTuple,
        tuple2::Tuple2,
        Float,
        Signed,
    };
    use crate::maths::traits::{
        Abs,
        Ceil,
        Floor,
    };
    use crate::tup;

    #[test]
    fn new() {
        let tup = tup!(42, 16);
        assert!(tup.x == 42);
        assert!(tup.y == 16);
    }

    #[test]
    fn ndim() {
        assert!(Tuple2::<i32>::ndim() == 2);
    }

    #[test]
    fn eq() {
        let tup_a = tup!(10, 20);
        let tup_b = tup_a.clone();
        assert_eq!(tup_a, tup_b);

        let tup_c = tup!(1, 2);
        let tup_d = tup!(3, 4);
        assert_ne!(tup_c, tup_d);
    }

    #[test]
    fn add_tup() {
        let tup_a = tup!(10, 20);
        let tup_b = tup!(30, 40);
        let expected = tup!(40, 60);
        assert_eq!(tup_a + tup_b, expected);
    }

    #[test]
    fn add_t() {
        let tup = tup!(10, 20);
        assert_eq!(tup + 5, tup!(15, 25));
    }

    #[test]
    fn add_assign_tup() {
        let mut tup = tup!(1, 2);
        tup += tup!(3, 4);
        assert_eq!(tup, tup!(4, 6));
    }

    #[test]
    fn add_assign_t() {
        let mut tup = tup!(1, 2);
        tup += 5;
        assert_eq!(tup, tup!(6, 7));
    }

    #[test]
    fn sub_tup() {
        let tup_a = tup!(50, 25);
        let tup_b = tup!(25, 12);
        let expected = tup!(25, 13);
        assert_eq!(tup_a - tup_b, expected);
    }

    #[test]
    fn sub_t() {
        let tup = tup!(10, 20);
        assert_eq!(tup - 5, tup!(5, 15));
    }

    #[test]
    fn sub_assign_tup() {
        let mut tup = tup!(50, 25);
        tup -= tup!(25, 12);
        assert_eq!(tup, tup!(25, 13));
    }

    #[test]
    fn sub_assign_t() {
        let mut tup = tup!(10, 20);
        tup -= 5;
        assert_eq!(tup, tup!(5, 15));
    }

    #[test]
    fn mul_tup() {
        let tup_a = tup!(1, 2);
        let tup_b = tup!(3, 4);
        assert_eq!(tup_a * tup_b, tup!(3, 8));
    }

    #[test]
    fn mul_t() {
        let tup = tup!(1, 2);
        assert_eq!(tup * 5, tup!(5, 10));
    }

    #[test]
    fn mul_assign_tup() {
        let mut tup = tup!(1, 2);
        tup *= tup!(3, 4);
        assert_eq!(tup, tup!(3, 8));
    }

    #[test]
    fn mul_assign_t() {
        let mut tup = tup!(1, 2);
        tup *= 5;
        assert_eq!(tup, tup!(5, 10));
    }

    #[test]
    fn div_tup() {
        let tup_a = tup!(24, 12);
        let tup_b = tup!(2, 4);
        assert_eq!(tup_a / tup_b, tup!(12, 3));
    }

    #[test]
    fn div_t() {
        let tup = tup!(10, 20);
        assert_eq!(tup / 5, tup!(2, 4))
    }

    #[test]
    fn div_assign_tup() {
        let mut tup = tup!(24, 12);
        tup /= tup!(2, 4);
        assert_eq!(tup, tup!(12, 3));
    }

    #[test]
    fn div_assign_t() {
        let mut tup = tup!(24, 12);
        tup /= 2;
        assert_eq!(tup, tup!(12, 6));
    }

    #[test]
    fn index() {
        let mut tup = tup!(1, 2);
        tup[0] = 3;
        tup[1] = 4;
        assert_eq!(tup[0], 3);
        assert_eq!(tup[1], 4);
    }

    #[test]
    #[should_panic]
    fn index_panic() {
        let tup = tup!(1, 2);
        tup[2];
    }

    #[test]
    fn iter() {
        let mut iter = tup!(1, 2).into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn abs() {
        // let tup = tup!(32, -84; i32);
        let tup = Tuple2::<i32>::new(32, -84);
        assert_eq!(tup.abs(), tup!(32, 84));
    }

    #[test]
    fn ceil() {
        let tup = Tuple2::<f32>::new(3.14, 15.92);
        // assert_eq!(tup.ceil(), Tuple2::<f32>::new(4.0, 16.0));
    }

    #[test]
    fn floor() {
        let tup = Tuple2::<f32>::new(3.14, 15.92);
        // assert_eq!(tup.floor(), Tuple2::<f32>::new(3.0, 15.0));
    }

    #[test]
    fn lerp() {
        let a = tup!(1.0, 2.0);
        let b = tup!(4.0, 5.0);
        // assert_eq!(Tuple2::<f32>::lerp(&a, &b, 0.75), tup!(1.0, 2.0));
    }

    #[test]
    fn neg() {
        // assert_eq!(-tup!(1, 2), tup!(-1, -2));
    }
}


#[cfg(test)]
mod tuple3_tests {
    use super::{
        BaseTuple,
        FloatTuple,
        SignedTuple,
        tuple3::Tuple3,
    };

    #[test]
    fn new() {
        let tup = tup!(13, 51, 79);
        assert_eq!(tup.x, 13);
        assert_eq!(tup.y, 51);
        assert_eq!(tup.z, 79);
    }

    #[test]
    fn ndim() {
        assert_eq!(Tuple3::<i32>::ndim(), 3);
    }

    #[test]
    fn eq() {
        let tup = tup!(13, 51, 79);
        assert_eq!(tup, tup!(13, 51, 79));
        assert_ne!(tup, tup!(13, 51, 80))
    }

    #[test]
    fn add_tup() {
        let tup_a = tup!(1, 2, 3);
        let tup_b = tup!(4, 5, 6);
        assert_eq!(tup_a + tup_b, tup!(5, 7, 9));
    }

    #[test]
    fn add_t() {
        let tup = tup!(1, 2, 3);
        assert_eq!(tup + 5, tup!(6, 7, 8));
    }

    #[test]
    fn add_assign_tup() {
        let mut tup = tup!(1, 2, 3);
        tup += tup!(4, 5, 6);
        assert_eq!(tup, tup!(5, 7, 9));
    }

    #[test]
    fn add_assign_t() {
        let mut tup = tup!(1, 2, 3);
        tup += 5;
        assert_eq!(tup, tup!(6, 7, 8));
    }

    #[test]
    fn sub_tup() {
        let tup_a = tup!(10, 20, 30);
        let tup_b = tup!(4, 5, 6);
        let expected = tup!(6, 15, 24);
        assert_eq!(tup_a - tup_b, expected);
    }

    #[test]
    fn sub_t() {
        let tup = tup!(10, 20, 30);
        assert_eq!(tup - 5, tup!(5, 15, 25));
    }

    #[test]
    fn sub_assign_tup() {
        let mut tup = tup!(10, 20, 30);
        tup -= tup!(4, 5, 6);
        assert_eq!(tup, tup!(6, 15, 24));
    }

    #[test]
    fn sub_assign_t() {
        let mut tup = tup!(10, 20, 30);
        tup -= 5;
        assert_eq!(tup, tup!(5, 15, 25));
    }

    #[test]
    fn mul_tup() {
        let tup_a = tup!(1, 2, 3);
        let tup_b = tup!(4, 5, 6);
        assert_eq!(tup_a * tup_b, tup!(4, 10, 18));
    }

    #[test]
    fn mul_t() {
        let tup = tup!(1, 2, 3);
        assert_eq!(tup * 5, tup!(5, 10, 15));
    }

    #[test]
    fn mul_assign_tup() {
        let mut tup = tup!(1, 2, 3);
        tup *= tup!(4, 5, 6);
        assert_eq!(tup, tup!(4, 10, 18));
    }

    #[test]
    fn mul_assign_t() {
        let mut tup = tup!(1, 2, 3);
        tup *= 5;
        assert_eq!(tup, tup!(5, 10, 15));
    }

    #[test]
    fn div_tup() {
        let tup_a = tup!(12, 24, 36);
        let tup_b = tup!(3, 4, 9);
        assert_eq!(tup_a / tup_b, tup!(4, 6, 4))
    }

    #[test]
    fn div_t() {
        let tup = tup!(12, 24, 36);
        assert_eq!(tup / 3, tup!(4, 8, 12));
    }

    #[test]
    fn div_assign_tup() {
        let mut tup = tup!(12, 24, 36);
        tup /= tup!(3, 4, 18);
        assert_eq!(tup, tup!(4, 6, 2));
    }

    #[test]
    fn div_assign_t() {
        let mut tup = tup!(12, 24, 36);
        tup /= 3;
        assert_eq!(tup, tup!(4, 8, 12));
    }

    #[test]
    fn index() {
        let mut tup = tup!(1, 2, 3);
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
        let tup = tup!(1, 2, 3);
        tup[3];
    }

    #[test]
    fn iter() {
        let mut iter = tup!(1, 2, 3).into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn abs() {
        let tup = tup!(32, -84, -121);
        // assert_eq!(tup.abs(), tup!(32, 84, 121));
    }

    #[test]
    fn ceil() {
        let tup = Tuple3::<f64>::new(12.51, 89.5, 16.0);
        // assert_eq!(tup.ceil(), Tuple3::<f64>::new(13.0, 90.0, 16.0));
    }

    #[test]
    fn floor() {
        let tup = Tuple3::<f64>::new(12.51, 89.5, 16.0);
        // assert_eq!(tup.floor(), Tuple3::<f64>::new(12.0, 89.0, 16.0));
    }

    #[test]
    fn neg() {
        // assert_eq!(-tup!(1, 2, 3), tup!(-1, -2, -3));
    }
}
