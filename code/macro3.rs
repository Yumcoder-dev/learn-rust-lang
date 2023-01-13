fn main() {
    // let a = my_vec!(1, 2, 3); ->
    let a = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };
    dbg!(a);
}
