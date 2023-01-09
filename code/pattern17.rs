let x = Some(5);
let y = 10;
match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {n}"),
    _ => println!("Default case, x = {:?}", x),
}
println!("at the end: x = {:?}, y = {y}", x);

let x1 = 4;
let y1 = false;
match x1 {
    4 | 5 | 6 if y1 => println!("yes"),
    _ => println!("no"),
}