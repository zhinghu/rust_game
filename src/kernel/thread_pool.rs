pub struct ThreadPool<F>
where
    F: Fn() + 'static,
{
    worker: Vec<F>,
}

impl<F> ThreadPool<F>
where
    F: Fn() + 'static,
{
    pub fn new() -> ThreadPool<F> {
        ThreadPool { worker: vec![] }
    }

    pub fn add(&mut self, f: F) {
        self.worker.push(f);
    }

    pub fn run(&self) {
        self.worker.into_iter().map(|&f| f());
    }

    /// Loop Run
    pub fn LRun(&self) {}
}
