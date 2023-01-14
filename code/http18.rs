impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // New!

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
