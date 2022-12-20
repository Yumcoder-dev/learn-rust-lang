fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}"); // 6.4

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}
