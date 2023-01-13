#![allow(unused)]
fn main() {
    union MyUnion {
        f1: u32,
        f2: f32,
    }

    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f1 };
}
// see https://doc.rust-lang.org/reference/items/unions.html
