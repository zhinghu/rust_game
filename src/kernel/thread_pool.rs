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
        for f in &self.worker {
            f();
        }
    }

    /// Loop Run
    pub fn lrun(&self) {}
}
