fn main() {
    let mut s0 = String::new();

    let data = "initial contents";
    let s1 = data.to_string();

    // the method also works on a literal directly:
    let s2 = "initial contents".to_string();

    let s3 = String::from("initial contents");
}
