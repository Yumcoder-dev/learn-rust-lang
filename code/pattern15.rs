fn main() {
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}
