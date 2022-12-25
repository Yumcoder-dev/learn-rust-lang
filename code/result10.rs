use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    // Reading a file into a string is a fairly common operation, so the standard library provides the convenient fs::read_to_string function.
    fs::read_to_string("hello.txt")
}
