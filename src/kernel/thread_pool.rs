pub struct ThreadPool {
    threads: Vec<std::thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new<F>(mut thread_count: i32, fun: F) -> ThreadPool
    where
        F: Fn() -> () + std::marker::Send + 'static,
    {
        let mut TP = ThreadPool { threads: vec![] };

        if (thread_count == 0) {
            thread_count = 1;
        }

        for i in 0..thread_count {
            TP.threads.push(std::thread::spawn(fun));
        }

        TP
    }
}
