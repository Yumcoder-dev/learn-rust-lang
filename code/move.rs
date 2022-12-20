fn main() {
    let x = 5;
    let y = x;
    func(x); // y = x, func paramater bind on stack
}
fn func(y: u8) {
    y
}
