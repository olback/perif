use std::{
    thread::JoinHandle,
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            Ordering
        }
    }
};

#[derive(Debug)]
pub struct TaskHandler {
    should_stop: Arc<AtomicBool>,
    threads: Option<Vec<JoinHandle<()>>>
}

impl TaskHandler {

    pub fn new() -> TaskHandler {

        TaskHandler {
            should_stop: Arc::new(AtomicBool::new(false)),
            threads: Some(Vec::new())
        }

    }

    pub fn get_bool(&self) -> Arc<AtomicBool> {

        Arc::clone(&self.should_stop)

    }

    pub fn add(&mut self, thread: JoinHandle<()>) {

        match self.threads.as_mut() {
            Some(threads) => threads.push(thread),
            None => eprintln!("Threads already killed!")
        }

    }

    pub fn stop_all(&mut self) {

        self.should_stop.swap(true, Ordering::Acquire);

        match self.threads.take() {
            Some(threads) => {
                for t in threads {
                    t.join().unwrap()
                }
            },
            None => eprintln!("Threads already killed!")
        }

    }

}
