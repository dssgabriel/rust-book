use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// ThreadPool data structure
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

// Message enum for graceful shutdown and cleanup
enum Message {
    NewJob(Job),
    Terminate
}

// Implementation of ThreadPool data structure
impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); 
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); 
        }
        return ThreadPool { 
            workers, 
            sender 
        };
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
} 

// Implementation of Drop trait
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Worker data structure for ThreadPool
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

// Implementation of Worker data structure
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        return Worker {
            id, 
            thread: Some(thread) 
        };
    }
}
