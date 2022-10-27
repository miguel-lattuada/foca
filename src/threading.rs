use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|id| Worker::new(id, receiver.clone()))
            .collect();

        ThreadPool { workers, sender }
    }

    pub fn execute(&self, f: impl FnOnce() + Send + 'static) {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let receiver = receiver.lock().expect("[MUTEX::POISONED]");
            let job = receiver.recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job.call_box();
        });

        Worker { id, thread }
    }
}
