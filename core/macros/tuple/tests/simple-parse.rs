use tuple_derive::Tuple2;
use tuple_trait::{ Tuple2, Numeric };

#[derive(Tuple2)]
pub struct Tuple2Thing<T: Numeric> {
    x: T,
    y: T,
}

fn main() {}
