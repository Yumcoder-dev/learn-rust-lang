// while let Conditional Loops
et mut stack = Vec::new();
stack.push(1);
// ---snip---
while let Some(top) = stack.pop() {
    println!("{}", top);
}

// for Loops
let v = vec!['a', 'b', 'c'];
for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}

// let Statements
// let PATTERN = EXPRESSION;
let (x, y, z) = (1, 2, 3);