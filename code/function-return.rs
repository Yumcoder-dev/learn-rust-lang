fn five() -> i32 {
    5 // no semicolon because it’s an expression whose value we want to return
}

fn main() {
    let x = five();

    println!("The value of x is: {x}"); // 6
}
