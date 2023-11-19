use tuple_derive::{ Tuple2, coordinates };
use tuple_trait::{ Tuple2, Numeric };

#[derive(Tuple2)]
#[coordinates(x = "x", y = "y")]
pub struct Tuple2Thing<T: Numeric> {
    x: T,
    y: T,
}

fn main() {}
