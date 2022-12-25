struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // The fields x and y must be the same type because both have the same generic data type T
    // let wont_work = Point { x: 5, y: 4.0 }; // expected integer, found floating-point number
    // struct Point<T, U> {
    //     x: T,
    //     y: U,
    // }
}
