pub struct ThreadPool;

impl ThreadPool {

    /// creates a new ThreadPool.
    /// 
    /// 'size' is the number of threads in the pool.
    /// 
    /// # panics
    /// this function will panic if size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {

        }
}