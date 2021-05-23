use std::{
    error::Error,
    fmt::{
        Formatter,
        Display,
        Result as Fmt_result
    }
};

pub struct ThreadPool;

#[derive(Debug)]
pub struct ThreadPoolError;

impl Error for ThreadPoolError {
    fn description(&self) -> &str {
        "yikes my guy"
    }
}

impl Display for ThreadPoolError {
    fn fmt(&self, f: &mut Formatter) -> Fmt_result {
        write!(f, "Whoops!")
    }
}

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// `size`: the number of threads in the pool
    ///
    /// # Panics
    ///
    /// Will panic if the size is 0
    ///
    pub fn new (size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size > 0 {
            return Err(ThreadPoolError)
        }

        Ok(ThreadPool)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {
        }
}
