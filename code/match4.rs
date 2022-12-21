fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
//  Error! We didn’t handle the None case, so this code will cause a bug. Rust knows that we didn’t cover every possible case and even knows which pattern we forgot!
// 3   ~   Some(i) => Some(i + 1),
// 4   ~   None => todo!(),
