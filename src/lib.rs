use std::{thread, sync::{mpsc, Mutex, Arc}}; //mpsc is a module for creating threads and sending and recieving data from on to the other

/// Threadpool is a struct for storing workers which contain ids and threads
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

// type alias for this so all types of jobs can be based
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    /// size is number of thread s in pool 
    /// 
    /// # Panics
    /// the `new` function panics if size is zero 
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // put it into a smart pointer so you can mutate it from anywhere
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver))); // reciever given to the Worker so each thread can recieve something
        }

        ThreadPool {workers, sender}
    } 

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),   // closure trait bound  which takes ownership of values in environemnt
        F: Send + 'static,   // send trait allows you to transfer the closure from one thread to another 
                             //  'static allows reiciever to hold onto this trait as long as needs
    {
        // want to send our closure through to the thread based on the jobs
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

/// Worker struct is used to store an id and the thread, for better debugging this is what we will store
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    /// Creats a new worker 
    /// id gives it a unique name so you know which worker is workers 
    /// recviever is a smart pointer to a reciever 
    /// 
    /// The function looks for jobs from the reciever and mutexs and gets it out and then executes it inside the thread
    fn new(id:usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Here we spawn a new thread where we move the closure into the thread and loop so we can keep recieving
        let thread = thread::spawn(move || loop {
            let job = reciever
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            println!("Worker {}, got a job; Executing.", id);
            job();
        });
        Worker {id, thread}
    }
}