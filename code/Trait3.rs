pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
pub fn notify(item: &(impl Summary + Display)) {}
// The + syntax is also valid with trait bounds on generic types:
pub fn notify<T: Summary + Display>(item: &T) {}
