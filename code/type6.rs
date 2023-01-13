fn generic<T>(t: T) {
    // --snip--
}
// is actually treated as though we had written this:
fn generic<T: Sized>(t: T) {
    // --snip--
}
