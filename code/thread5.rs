use std::sync::mpsc;
use std::thread;
fn main() {
    // The mpsc::channel function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver. The abbreviations tx and rx are traditionally used in many fields for transmitter and receiver respectively.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
