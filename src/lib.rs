use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool{
    workers: Vec<Worker>,
    senders: mpsc::Sender<Job>,
}
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    impl ThreadPool{
        pub fn new(size: usize) -> ThreadPool{
            assert!(size > 0);
            
            let(senders, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size{
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }


            ThreadPool {workers, senders}
        }

        pub fn excecute<F>(&self, f: F)
        where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.senders.send(job).unwrap();
    }

}
struct Worker{
    id: usize,
    thread: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move|| loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("woker {} Got a job; excecuting", id);

            job();
        });
            
        Worker{id, thread}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

