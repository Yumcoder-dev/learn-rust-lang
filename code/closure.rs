// a function definition
fn  add_one_v1   (x: u32) -> u32 { x + 1 }; 
// a fully annotated closure definition
let add_one_v2 = |x: u32| -> u32 { x + 1 }; 
// we remove the type annotations from the closure definition
let add_one_v3 = |x|             { x + 1 }; 
// we remove the brackets, which are optional because the closure body has only one expression
let add_one_v4 = |x|               x + 1  ; 

let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5); // Error: Attempting to call a closure whose types are inferred with two different types