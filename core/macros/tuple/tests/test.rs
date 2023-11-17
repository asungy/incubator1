#[test]
fn parse() {
    let t = trybuild::TestCases::new();
    t.pass("tests/simple-parse.rs");
}

#[cfg(test)]
mod tuple2 {
    use tuple_derive::Tuple2;
    use tuple_trait::{ Tuple2, Numeric };

    #[derive(Tuple2)]
    struct Tup2<T: Numeric> {
        x: T,
        y: T,
    }

    #[test]
    fn ndim() {
        assert_eq!(Tup2::<i32>::ndim(), 2);
    }
}
