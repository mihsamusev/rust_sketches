use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, RecvError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{mem, thread};

type Task = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, Clone)]
pub struct Pool {
    handles: Arc<VecDeque<JoinHandle<()>>>,

    // Sender will send tasks to threads
    sender: Option<Sender<Task>>,
}

impl Pool {
    pub fn with_thread_count(n: usize) -> Option<Self> {
        if n == 0 {
            return None;
        }

        // thread communication
        let (sender, rx) = mpsc::channel::<Task>();

        // Receiver<T> !Sync -> needs mutex
        let rx = Arc::new(Mutex::new(rx));

        // generate hanging threads with receivers
        let handles = (0..n)
            .map(|_| {
                let rx = rx.clone();
                thread::spawn(move || {
                    while let Ok(t) = get_task(&rx) {
                        // thread is blocked if no task
                        t()
                    }
                })
            })
            .collect();

        Some(Self {
            handles: Arc::new(handles),
            sender: Some(sender),
        })
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        // sender .as_ref() does &Option<T> to Option<&T>
        let _ = self.sender.as_ref().unwrap().send(task);
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        // will own arc and put default vector into self.handles
        //let arc = mem::replace(&mut self.handles, Arc::new(Default::default()));
        let arc = mem::take(&mut self.handles);

        // success if its the last copy
        if let Ok(handles) = Arc::try_unwrap(arc) {
            mem::drop(self.sender.take()); // if sender lives the receiver will block thread
            for jh in handles {
                jh.join().unwrap();
            }
        }
    }
}

pub fn get_task(rx: &Mutex<Receiver<Task>>) -> Result<Task, RecvError> {
    // blocks the thread if no task
    if let Ok(inner) = rx.lock() {
        inner.recv().map_err(|_| RecvError)
    } else {
        Err(RecvError)
    }
}
