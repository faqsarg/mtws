use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // add threads to the vector
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {

        }
}