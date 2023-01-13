use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let x: u32 = 5;
    let y: Millimeters = Millimeters(10);
    println!("x + y = {}", x + y);
    //                     ^ no implementation for `u32 + Millimeters`
}
