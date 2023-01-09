pub trait Draw {
    fn draw(&self);
}
// pub struct Screen {
//     // vector of trait objects (dynamic)
//     pub components: Vec<Box<dyn Draw>>,
// }
pub struct Screen<T: Draw> {
    // generic type parameter with trait bounds (static)
    pub components: Vec<T>,
}
impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
