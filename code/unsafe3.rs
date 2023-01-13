let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..]) 
// | --------------------------^^^^^^--------
// | |     |                   |
// | |     |                   second mutable borrow occurs here
// | |     first mutable borrow occurs here
// | returning this value requires that `*values` is borrowed for `'1`
}