fn main() {
    let v1: Vec<i32> = Vec::new(); // We added a type annotation here, because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store

    // More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation. Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
}
