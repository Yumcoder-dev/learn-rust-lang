#![feature(log_syntax)]
// #![feature(trace_macros)]
// trace_macros!(true);

#[macro_export]
macro_rules! my_vec {
    ($( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                log_syntax!(expr=$x); // debug
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
fn main() {
    let a = my_vec!(1, 2, 3); // my_vec![1, 2, 3] or my_vec!{1, 2, 3]}
    dbg!(a);
}
