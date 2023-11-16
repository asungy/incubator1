use crate::maths::traits::tuple::Tuple2;
use tuple_derive::Tuple2;

#[derive(Tuple2)]
struct Point;

#[cfg(test)]
mod tests {
    use super::{ Point, Tuple2 };

    #[test]
    fn hello() {
        assert_eq!(Point::hello(), String::from("Hello, Point!"));
    }
}
