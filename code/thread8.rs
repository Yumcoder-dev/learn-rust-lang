// producer 2
thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        // --snip--
    ];
    for val in vals {
        tx.send(val).unwrap();
        // --snip--
    }
});

for received in rx {
    println!("Got: {}", received);
}