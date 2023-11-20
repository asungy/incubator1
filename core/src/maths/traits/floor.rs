pub trait Floor {
    type Item;
    fn floor(t: Self::Item) -> Self::Item;
}

macro_rules! impl_floor_int {
    ($t:ty) => {
        impl Floor for $t {
            type Item = $t;
            fn floor(t: Self::Item) -> Self::Item {
                t
            }
        }
    };
}

macro_rules! impl_floor_float {
    ($t:ty) => {
        impl Floor for $t {
            type Item = $t;
            fn floor(t: Self::Item) -> Self::Item {
                t.floor()
            }
        }
    };
}

impl_floor_int!(u8);
impl_floor_int!(u16);
impl_floor_int!(u32);
impl_floor_int!(u64);
impl_floor_int!(u128);
impl_floor_int!(usize);
impl_floor_int!(i8);
impl_floor_int!(i16);
impl_floor_int!(i32);
impl_floor_int!(i64);
impl_floor_int!(i128);
impl_floor_int!(isize);

impl_floor_float!(f32);
impl_floor_float!(f64);
