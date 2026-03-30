use std::{sync::mpsc, thread};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

struct Job;

impl Worker {
    fn new (id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }

    }
}
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


impl ThreadPool {
    /// create a new threadpool
    /// size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The new function will panic if the size is 0
    /// 
    
    pub fn new(size: usize) -> ThreadPool {

        assert!(size > 0);
        let (sender, receiver) =  mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }
         ThreadPool {
            workers, sender
         }
    }

    pub fn execute<F> (&self, f:F) 
    where 
         F: FnOnce() + Send + 'static,
    {

    }
}