struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

trait Animal {
    fn baby_name() -> String;
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name()); // Spot
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // puppy
}
