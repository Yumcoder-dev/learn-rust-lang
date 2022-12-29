use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: add some error handling to deal with certain potential erroneous situations
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
