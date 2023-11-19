pub trait Floor {
    type Item;
    fn floor(t: Self::Item) -> Self::Item;
}

impl Floor for u8 {
    type Item = u8;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for u16 {
    type Item = u16;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for u32 {
    type Item = u32;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for u64 {
    type Item = u64;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for u128 {
    type Item = u128;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for usize {
    type Item = usize;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for i8 {
    type Item = i8;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for i16 {
    type Item = i16;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for i32 {
    type Item = i32;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for i64 {
    type Item = i64;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for i128 {
    type Item = i128;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for isize {
    type Item = isize;

    fn floor(t: Self::Item) -> Self::Item {
        t
    }
}

impl Floor for f32 {
    type Item = f32;

    fn floor(t: Self::Item) -> Self::Item {
        t.floor()
    }
}

impl Floor for f64 {
    type Item = f64;

    fn floor(t: Self::Item) -> Self::Item {
        t.floor()
    }
}
