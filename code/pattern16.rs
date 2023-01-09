struct Point {
    x: i32,
    y: i32,
    z: i32,
}
let origin = Point { x: 0, y: 0, z: 0 };
match origin {
    Point { x, .. } => println!("x is {}", x),
}

let numbers = (2, 4, 8, 16, 32);
match numbers {
    (first, .., last) => {
        println!("Some numbers: {first}, {last}");
    }
}