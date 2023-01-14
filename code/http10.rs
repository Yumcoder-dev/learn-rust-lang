struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
    }
}
