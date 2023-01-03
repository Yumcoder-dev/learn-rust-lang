fn main() {
    let i = 30;
    {
        let b = Box::new(i);
        dbg!(b);
    }
    dbg!(i);
}
