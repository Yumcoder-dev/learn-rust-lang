impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
            // ^^^^^^^^`worker.thread` moved due to this method call
            // join function takes ownership of the receiver `self`, which moves `worker.thread`
        }
    }
}
