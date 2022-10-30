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

pub enum Message {
    Process(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|id| Worker::new(id, receiver.clone()))
            .collect();

        ThreadPool { workers, sender }
    }

    pub fn execute(&self, f: impl FnOnce() + Send + 'static) {
        let job = Box::new(f);
        self.sender.send(Message::Process(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let receiver = receiver.lock().expect("[MUTEX::POISONED]");

            if let Ok(message) = receiver.recv() {
                drop(receiver);

                match message {
                    Message::Process(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job.call_box();
                        println!("Worker {} finished job", id);
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
