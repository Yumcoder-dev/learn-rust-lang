type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;
// Because Kilometers and i32 are the same type, we can add values of both types and we can pass Kilometers values to functions that take i32 parameters. 
println!("x + y = {}", x + y);