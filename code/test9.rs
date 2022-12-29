// Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
