fn first_word(s: &str) -> &str {} // Ok, lifetime elision rules are applied!
// Then the compiler applies the first rule, which specifies that each parameter gets its own lifetime. We’ll call it 'a as usual, so now the signature is this:
fn first_word<'a>(s: &'a str) -> &str {}
// The second rule applies because there is exactly one input lifetime. The second rule specifies that the lifetime of the one input parameter gets assigned to the output lifetime, so the signature is now this:
fn first_word<'a>(s: &'a str) -> &'a str {}
//
// example 2:
fn longest(x: &str, y: &str) -> &str {}
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
// You can see that the second rule doesn’t apply because there is more than one input lifetime. The third rule doesn’t apply either, because longest is a function rather than a method, so none of the parameters are self. After working through all three rules, we still haven’t figured out what the return type’s lifetime is. This is why we got an error trying to compile the code.
