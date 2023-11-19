pub trait Abs {
    type Item;
    fn abs(t: Self::Item) -> Self::Item;
}

impl Abs for u8 {
    type Item = u8;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for u16 {
    type Item = u16;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for u32 {
    type Item = u32;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for u64 {
    type Item = u64;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for u128 {
    type Item = u128;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for usize {
    type Item = usize;

    fn abs(t: Self::Item) -> Self::Item {
        t
    }
}

impl Abs for i8 {
    type Item = i8;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for i16 {
    type Item = i16;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for i32 {
    type Item = i32;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for i64 {
    type Item = i64;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for i128 {
    type Item = i128;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for isize {
    type Item = isize;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for f32 {
    type Item = f32;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}

impl Abs for f64 {
    type Item = f64;

    fn abs(t: Self::Item) -> Self::Item {
        t.abs()
    }
}
