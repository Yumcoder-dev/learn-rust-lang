struct NaiveRc<T> {
    inner_value: T,
    references: Cell<usize>,
}
impl<T: Clone> Clone for NaiveRc<T> {
    // Clone trait signature accept a reference(&self) as argument, so who we can change fields inside our Struct?
    // Note that it does not accept a mutate/unique reference (&mut)
    fn clone(&self) -> Self {
        // mutate references' value through self!
        self.references.set(self.references.get() + 1);
        NaiveRc {
            inner_value: self.inner_value.clone(),
            references: self.references.clone(),
        }
    }
}

fn main() {}
