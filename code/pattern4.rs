// |     let Some(x) = some_option_value;
// |         ^^^^^^^ pattern `None` not covered

// If we have a refutable pattern where an irrefutable pattern is needed, we can fix it by changing the code that uses the pattern: instead of using let, we can use if let. 
if let Some(x) = some_option_value {
    println!("{}", x);
}