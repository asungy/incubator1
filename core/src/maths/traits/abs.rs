pub trait Abs {
    type Item;
    fn abs(t: Self::Item) -> Self::Item;
}
