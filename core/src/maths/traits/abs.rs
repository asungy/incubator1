pub trait Abs {
    type Item;
    fn abs(t: Self::Item) -> Self::Item;
}

macro_rules! impl_abs_unsigned {
    ($t:ty) => {
        impl Abs for $t {
            type Item = $t;
            fn abs(t: Self::Item) -> Self::Item {
                t
            }
        }
    };
}

macro_rules! impl_abs_signed {
    ($t:ty) => {
        impl Abs for $t {
            type Item = $t;
            fn abs(t: Self::Item) -> Self::Item {
                t.abs()
            }
        }
    };
}

impl_abs_unsigned!(u8);
impl_abs_unsigned!(u16);
impl_abs_unsigned!(u32);
impl_abs_unsigned!(u64);
impl_abs_unsigned!(u128);
impl_abs_unsigned!(usize);

impl_abs_signed!(i8);
impl_abs_signed!(i16);
impl_abs_signed!(i32);
impl_abs_signed!(i64);
impl_abs_signed!(i128);
impl_abs_signed!(isize);
impl_abs_signed!(f32);
impl_abs_signed!(f64);
