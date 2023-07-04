fn main() {
    generic_data_type();
}

fn generic_data_type() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let boolean: Point<bool> = Point { x: true, y: false };
    let integer: Point<i32> = Point { x: 1, y: 9 };
    let float: Point<f64> = Point { x: 1.7, y: 4.3 };
    let string_slice: Point<&str> = Point {
        x: "high",
        y: "low",
    };
    // let mix = Point { x: 1.7, y: false };

    #[derive(Debug)]
    struct Point2<T, U> {
        x: U,
        y: T,
    }

    let integer_and_boolean: Point2<bool, i32> = Point2 { x: 5, y: false };

    println!(
        "{:?} {:?} {:?} {:?} {:?} {:?}",
        boolean.x, integer.y, float, string_slice, integer_and_boolean.x, integer_and_boolean.y
    )
}
