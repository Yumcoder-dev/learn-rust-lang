use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", file_path);

    // TODO: handling errors as well as we could
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
