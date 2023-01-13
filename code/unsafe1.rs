let mut num = 5;
// Create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}