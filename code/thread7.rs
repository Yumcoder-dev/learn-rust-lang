// --snip--
let (tx, rx) = mpsc::channel();
// producer 1
let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        // --snip--
    ];
    for val in vals {
        tx1.send(val).unwrap();
        // --snip--
    }
});