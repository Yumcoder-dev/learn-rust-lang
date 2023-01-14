impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // Chanage!
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
