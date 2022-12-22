fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50; // we have to use the * dereference operator to get to the value in i before we can use the += operator
    }
}
