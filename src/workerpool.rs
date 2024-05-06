use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send>;

pub struct WorkerPool {
    sender: mpsc::Sender<Job>,
    workers: Vec<Worker>
}

impl WorkerPool {
    pub fn new(pool_size: usize) -> WorkerPool {
        let (sender, reciever) = mpsc::channel::<Job>();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(pool_size);
        for i in 0..pool_size {
            let reciever = reciever.clone();
            workers.push(Worker::new(i, reciever));
        }
        WorkerPool {
            sender,
            workers
        }
    }

    pub fn process(&self, job: Job) {
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    join_handle: JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let join_handle = thread::spawn(move || {
            while let Ok(job) = reciever.lock().unwrap().recv() {
                job();
            }
        });
        Worker {
            id,
            join_handle
        }
    }
}
