struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // Chanage!
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--
        Worker {
            id,
            thread: Some(thread), // Chanage!
        }
    }
}
