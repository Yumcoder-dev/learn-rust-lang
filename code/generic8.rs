let integer = Some(5);
let float = Some(5.0);
// The monomorphized version of the above code looks similar to the following (the compiler uses different names than what we’re using here for illustration):
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}