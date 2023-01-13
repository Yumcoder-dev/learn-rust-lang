fn generic<T: Sized>(t: T) {
    // --snip--
}

fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
