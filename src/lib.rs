use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {

    /// creates a new ThreadPool.
    /// 
    /// 'size' is the number of threads in the pool.
    /// 
    /// # panics
    /// this function will panic if size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {

        }
}

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker{
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}