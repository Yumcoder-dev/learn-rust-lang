fn returns_closure() -> dyn Fn(i32) -> i32 {
    //                  ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |x| x + 1
}

// The error references the Sized trait again! Rust doesn’t know how much space it will need to store the closure. We can use a trait object:
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
