pub trait Ceil {
    type Item;
    fn ceil(t: Self::Item) -> Self::Item;
}

macro_rules! impl_ceil_int {
    ($t:ty) => {
        impl Ceil for $t {
            type Item = $t;
            fn ceil(t: Self::Item) -> Self::Item {
                t
            }
        }
    };
}

macro_rules! impl_ceil_float {
    ($t:ty) => {
        impl Ceil for $t {
            type Item = $t;
            fn ceil(t: Self::Item) -> Self::Item {
                t.ceil()
            }
        }
    };
}

impl_ceil_int!(u8);
impl_ceil_int!(u16);
impl_ceil_int!(u32);
impl_ceil_int!(u64);
impl_ceil_int!(u128);
impl_ceil_int!(usize);
impl_ceil_int!(i8);
impl_ceil_int!(i16);
impl_ceil_int!(i32);
impl_ceil_int!(i64);
impl_ceil_int!(i128);
impl_ceil_int!(isize);

impl_ceil_float!(f32);
impl_ceil_float!(f64);
