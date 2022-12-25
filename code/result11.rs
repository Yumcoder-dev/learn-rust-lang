use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
    // Error!
    //  ^ cannot use the `?` operator in a function that returns `()`
    // help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
}
