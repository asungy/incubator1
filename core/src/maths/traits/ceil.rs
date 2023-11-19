pub trait Ceil {
    type Item;
    fn ceil(t: Self::Item) -> Self::Item;
}

impl Ceil for u8 {
    type Item = u8;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for u16 {
    type Item = u16;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for u32 {
    type Item = u32;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for u64 {
    type Item = u64;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for u128 {
    type Item = u128;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for usize {
    type Item = usize;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for i8 {
    type Item = i8;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for i16 {
    type Item = i16;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for i32 {
    type Item = i32;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for i64 {
    type Item = i64;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for i128 {
    type Item = i128;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for isize {
    type Item = i128;

    fn ceil(t: Self::Item) -> Self::Item {
        t
    }
}

impl Ceil for f32 {
    type Item = f32;

    fn ceil(t: Self::Item) -> Self::Item {
        t.ceil()
    }
}

impl Ceil for f64 {
    type Item = f64;

    fn ceil(t: Self::Item) -> Self::Item {
        t.ceil()
    }
}
