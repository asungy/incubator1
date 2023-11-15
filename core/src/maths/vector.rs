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
use std::iter::{ Iterator, IntoIterator };
use super::numeric::Numeric;

/// A generic vector.
pub trait Vector<T: Numeric, Rhs = Self, Output = Self>:
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
    /// Returns a vector where each of the components are of their respective absolute
    /// values.
    fn abs(&self) -> Self;
    /// Returns the number of elements in the vector.
    fn ndim() -> usize;
    /// Computes the dot product between two vectors.
    fn dot(&self, other: &Self) -> T;
}

#[derive(Debug, Clone, Copy)]
struct Vector2<T: Numeric> {
    pub x: T,
    pub y: T,
}

impl<T: Numeric> Vector2<T> {
    fn new(x: T, y: T) -> Self {
        Vector2 {
            x,
            y,
        }
    }
}

impl<T: Numeric> Vector<T> for Vector2<T> {
    fn ndim() -> usize {
        2
    }

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn abs(&self) -> Self {
        Vector2 {
            x: T::abs(self.x),
            y: T::abs(self.y),
        }
    }
}

struct Vector2Iter<T: Numeric> {
    vec: Vector2<T>,
    index: usize,
}

impl<T: Numeric> Iterator for Vector2Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.vec.x,
            1 => self.vec.y,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl<T: Numeric> IntoIterator for Vector2<T> {
    type Item = T;
    type IntoIter = Vector2Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Vector2Iter {
            vec: self,
            index: 0,
        }
    }
}

impl<T: Numeric> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Numeric> Add<Self> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Numeric> Add<T> for Vector2<T> {
    type Output = Vector2::<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T: Numeric> AddAssign<Self> for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Numeric> AddAssign<T> for Vector2<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<T: Numeric> Sub<Self> for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl <T: Numeric> Sub<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T: Numeric> SubAssign<Self> for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Numeric> SubAssign<T> for Vector2<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl<T: Numeric> Mul<Self> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Numeric> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Numeric> MulAssign<Self> for Vector2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Numeric> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Numeric> Div<Self> for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl <T: Numeric> Div<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Numeric> DivAssign<Self> for Vector2<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Numeric> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Numeric> Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds: vector length is {}, but the index is {}.", Vector2::<T>::ndim(), index),
        }
    }
}

impl <T: Numeric> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds: vector length is {}, but the index is {}.", Vector2::<T>::ndim(), index),
        }
    }
}

type Vector2i32 = Vector2<i32>;

#[derive(Debug, Clone, Copy)]
struct Vector3<T: Numeric> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Numeric> Vector3<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Vector3 {
            x,
            y,
            z,
        }
    }
}

impl<T: Numeric> Vector<T> for Vector3<T> {
    fn ndim() -> usize {
        3
    }

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn abs(&self) -> Self {
        Vector3 {
            x: T::abs(self.x),
            y: T::abs(self.y),
            z: T::abs(self.z),
        }
    }
}

struct Vector3Iter<T: Numeric> {
    vec: Vector3<T>,
    index: usize,
}

impl<T: Numeric> Iterator for Vector3Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.vec.x,
            1 => self.vec.y,
            2 => self.vec.z,
            _ => return None,
        };
        self.index += 1;
        Some(result)

    }
}

impl<T: Numeric> IntoIterator for Vector3<T> {
    type Item = T;
    type IntoIter = Vector3Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Vector3Iter {
            vec: self,
            index: 0,
        }
    }
}

impl<T: Numeric> PartialEq for Vector3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Numeric> Add<Self> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Numeric> Add<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T: Numeric> AddAssign<Self> for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Numeric> AddAssign<T> for Vector3<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl<T: Numeric> Sub<Self> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Numeric> Sub<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T: Numeric> SubAssign<Self> for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Numeric> SubAssign<T> for Vector3<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl<T: Numeric> Mul<Self> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Numeric> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Numeric> MulAssign<Self> for Vector3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Numeric> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Numeric> Div<Self> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Numeric> Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Numeric> DivAssign<Self> for Vector3<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<T: Numeric> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Numeric> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: vector length is {}, but the index is {}.", Vector3::<T>::ndim(), index),
        }
    }
}

impl<T: Numeric> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds: vector length is {}, but the index is {}.", Vector3::<T>::ndim(), index),
        }
    }
}

type Vector3i32 = Vector3<i32>;

#[cfg(test)]
mod vector2_tests {
    use super::{Vector, Vector2i32};

    #[test]
    fn new() {
        let vec = Vector2i32::new(42, 16);
        assert!(vec.x == 42);
        assert!(vec.y == 16);
    }

    #[test]
    fn ndim() {
        assert!(Vector2i32::ndim() == 2);
    }

    #[test]
    fn dot() {
        let vec_a = Vector2i32::new(1, 2);
        let vec_b = Vector2i32::new(3, 4);
        assert_eq!(vec_a.dot(&vec_b), 11);
    }

    #[test]
    fn eq() {
        let vec_a = Vector2i32::new(10, 20);
        let vec_b = vec_a.clone();
        assert_eq!(vec_a, vec_b);

        let vec_c = Vector2i32::new(1, 2);
        let vec_d = Vector2i32::new(3, 4);
        assert_ne!(vec_c, vec_d);
    }

    #[test]
    fn add_vec() {
        let vec_a = Vector2i32::new(10, 20);
        let vec_b = Vector2i32::new(30, 40);
        let expected = Vector2i32::new(40, 60);
        assert_eq!(vec_a + vec_b, expected);
    }

    #[test]
    fn add_t() {
        let vec = Vector2i32::new(10, 20);
        assert_eq!(vec + 5, Vector2i32::new(15, 25));
    }

    #[test]
    fn add_assign_vec() {
        let mut vec = Vector2i32::new(1, 2);
        vec += Vector2i32::new(3, 4);
        assert_eq!(vec, Vector2i32::new(4, 6));
    }

    #[test]
    fn add_assign_t() {
        let mut vec = Vector2i32::new(1, 2);
        vec += 5;
        assert_eq!(vec, Vector2i32::new(6, 7));
    }

    #[test]
    fn sub_vec() {
        let vec_a = Vector2i32::new(50, 25);
        let vec_b = Vector2i32::new(25, 12);
        let expected = Vector2i32::new(25, 13);
        assert_eq!(vec_a - vec_b, expected);
    }

    #[test]
    fn sub_t() {
        let vec = Vector2i32::new(10, 20);
        assert_eq!(vec - 5, Vector2i32::new(5, 15));
    }

    #[test]
    fn sub_assign_vec() {
        let mut vec = Vector2i32::new(50, 25);
        vec -= Vector2i32::new(25, 12);
        assert_eq!(vec, Vector2i32::new(25, 13));
    }

    #[test]
    fn sub_assign_t() {
        let mut vec = Vector2i32::new(10, 20);
        vec -= 5;
        assert_eq!(vec, Vector2i32::new(5, 15));
    }

    #[test]
    fn mul_vec() {
        let vec_a = Vector2i32::new(1, 2);
        let vec_b = Vector2i32::new(3, 4);
        assert_eq!(vec_a * vec_b, Vector2i32::new(3, 8));
    }

    #[test]
    fn mul_t() {
        let vec = Vector2i32::new(1, 2);
        assert_eq!(vec * 5, Vector2i32::new(5, 10));
    }

    #[test]
    fn mul_assign_vec() {
        let mut vec = Vector2i32::new(1, 2);
        vec *= Vector2i32::new(3, 4);
        assert_eq!(vec, Vector2i32::new(3, 8));
    }

    #[test]
    fn mul_assign_t() {
        let mut vec = Vector2i32::new(1, 2);
        vec *= 5;
        assert_eq!(vec, Vector2i32::new(5, 10));
    }

    #[test]
    fn div_vec() {
        let vec_a = Vector2i32::new(24, 12);
        let vec_b = Vector2i32::new(2, 4);
        assert_eq!(vec_a / vec_b, Vector2i32::new(12, 3));
    }

    #[test]
    fn div_t() {
        let vec = Vector2i32::new(10, 20);
        assert_eq!(vec / 5, Vector2i32::new(2, 4))
    }

    #[test]
    fn div_assign_vec() {
        let mut vec = Vector2i32::new(24, 12);
        vec /= Vector2i32::new(2, 4);
        assert_eq!(vec, Vector2i32::new(12, 3));
    }

    #[test]
    fn div_assign_t() {
        let mut vec = Vector2i32::new(24, 12);
        vec /= 2;
        assert_eq!(vec, Vector2i32::new(12, 6));
    }

    #[test]
    fn index() {
        let mut vec = Vector2i32::new(1, 2);
        vec[0] = 3;
        vec[1] = 4;
        assert_eq!(vec[0], 3);
        assert_eq!(vec[1], 4);
    }

    #[test]
    #[should_panic]
    fn index_panic() {
        let vec = Vector2i32::new(1, 2);
        vec[2];
    }

    #[test]
    fn iter() {
        let mut iter = Vector2i32::new(1, 2).into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn abs() {
        let vec = Vector2i32::new(32, -84);
        assert_eq!(vec.abs(), Vector2i32::new(32, 84));
    }
}


#[cfg(test)]
mod vector3_tests {
    use super::{ Vector3i32, Vector };

    #[test]
    fn new() {
        let vec = Vector3i32::new(13, 51, 79);
        assert_eq!(vec.x, 13);
        assert_eq!(vec.y, 51);
        assert_eq!(vec.z, 79);
    }

    #[test]
    fn ndim() {
        assert_eq!(Vector3i32::ndim(), 3);
    }

    #[test]
    fn dot() {
        let vec_a = Vector3i32::new(1, 2, 3);
        let vec_b = Vector3i32::new(4, 5, 6);
        assert_eq!(vec_a.dot(&vec_b), 32);
    }

    #[test]
    fn eq() {
        let vec = Vector3i32::new(13, 51, 79);
        assert_eq!(vec, Vector3i32::new(13, 51, 79));
        assert_ne!(vec, Vector3i32::new(13, 51, 80))
    }

    #[test]
    fn add_vec() {
        let vec_a = Vector3i32::new(1, 2, 3);
        let vec_b = Vector3i32::new(4, 5, 6);
        assert_eq!(vec_a + vec_b, Vector3i32::new(5, 7, 9));
    }

    #[test]
    fn add_t() {
        let vec = Vector3i32::new(1, 2, 3);
        assert_eq!(vec + 5, Vector3i32::new(6, 7, 8));
    }

    #[test]
    fn add_assign_vec() {
        let mut vec = Vector3i32::new(1, 2, 3);
        vec += Vector3i32::new(4, 5, 6);
        assert_eq!(vec, Vector3i32::new(5, 7, 9));
    }

    #[test]
    fn add_assign_t() {
        let mut vec = Vector3i32::new(1, 2, 3);
        vec += 5;
        assert_eq!(vec, Vector3i32::new(6, 7, 8));
    }

    #[test]
    fn sub_vec() {
        let vec_a = Vector3i32::new(10, 20, 30);
        let vec_b = Vector3i32::new(4, 5, 6);
        let expected = Vector3i32::new(6, 15, 24);
        assert_eq!(vec_a - vec_b, expected);
    }

    #[test]
    fn sub_t() {
        let vec = Vector3i32::new(10, 20, 30);
        assert_eq!(vec - 5, Vector3i32::new(5, 15, 25));
    }

    #[test]
    fn sub_assign_vec() {
        let mut vec = Vector3i32::new(10, 20, 30);
        vec -= Vector3i32::new(4, 5, 6);
        assert_eq!(vec, Vector3i32::new(6, 15, 24));
    }

    #[test]
    fn sub_assign_t() {
        let mut vec = Vector3i32::new(10, 20, 30);
        vec -= 5;
        assert_eq!(vec, Vector3i32::new(5, 15, 25));
    }

    #[test]
    fn mul_vec() {
        let vec_a = Vector3i32::new(1, 2, 3);
        let vec_b = Vector3i32::new(4, 5, 6);
        assert_eq!(vec_a * vec_b, Vector3i32::new(4, 10, 18));
    }

    #[test]
    fn mul_t() {
        let vec = Vector3i32::new(1, 2, 3);
        assert_eq!(vec * 5, Vector3i32::new(5, 10, 15));
    }

    #[test]
    fn mul_assign_vec() {
        let mut vec = Vector3i32::new(1, 2, 3);
        vec *= Vector3i32::new(4, 5, 6);
        assert_eq!(vec, Vector3i32::new(4, 10, 18));
    }

    #[test]
    fn mul_assign_t() {
        let mut vec = Vector3i32::new(1, 2, 3);
        vec *= 5;
        assert_eq!(vec, Vector3i32::new(5, 10, 15));
    }

    #[test]
    fn div_vec() {
        let vec_a = Vector3i32::new(12, 24, 36);
        let vec_b = Vector3i32::new(3, 4, 9);
        assert_eq!(vec_a / vec_b, Vector3i32::new(4, 6, 4))
    }

    #[test]
    fn div_t() {
        let vec = Vector3i32::new(12, 24, 36);
        assert_eq!(vec / 3, Vector3i32::new(4, 8, 12));
    }

    #[test]
    fn div_assign_vec() {
        let mut vec = Vector3i32::new(12, 24, 36);
        vec /= Vector3i32::new(3, 4, 18);
        assert_eq!(vec, Vector3i32::new(4, 6, 2));
    }

    #[test]
    fn div_assign_t() {
        let mut vec = Vector3i32::new(12, 24, 36);
        vec /= 3;
        assert_eq!(vec, Vector3i32::new(4, 8, 12));
    }

    #[test]
    fn index() {
        let mut vec = Vector3i32::new(1, 2, 3);
        vec[0] = 4;
        vec[1] = 5;
        vec[2] = 6;
        assert_eq!(vec[0], 4);
        assert_eq!(vec[1], 5);
        assert_eq!(vec[2], 6);
    }

    #[test]
    #[should_panic]
    fn index_panic() {
        let vec = Vector3i32::new(1, 2, 3);
        vec[3];
    }

    #[test]
    fn iter() {
        let mut iter = Vector3i32::new(1, 2, 3).into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn abs() {
        let vec = Vector3i32::new(32, -84, -121);
        assert_eq!(vec.abs(), Vector3i32::new(32, 84, 121));
    }
}
