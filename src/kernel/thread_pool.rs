pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, std::sync::Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn run<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<Job>>>,
    ) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker { id, thread }
    }
}
