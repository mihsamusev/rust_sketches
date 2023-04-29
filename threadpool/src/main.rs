use kata_threadpool1::Pool;
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

fn measure<T>(f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = f();
    let duration = Instant::now() - start;
    println!("Completed in {:?}", duration);
    result
}

fn pool_find(data: Arc<[u32]>, target: u32, pool: Pool, chunk_size: usize) -> Option<u32> {
    let (sender, receiver) = mpsc::channel();

    let n_chunks = data.len() / chunk_size;
    for i in 0..n_chunks {
        let sender = sender.clone();
        let data = data.clone();

        pool.spawn(move || {
            let chunk_start = i * chunk_size;
            let chunk_end = (i + 1) * chunk_size;
            let chunk_data = &data[chunk_start..chunk_end];
            let found = chunk_data
                .iter()
                .enumerate()
                .find(|(_, v)| **v == target)
                .map(|(i, _)| chunk_start + i);
            let _ = sender.send(found);
        });
    }

    for _ in 0..n_chunks {
        if let Some(found) = receiver.recv().unwrap() {
            return Some(found as u32);
        }
    }

    None
}

fn main() {
    let chunk_size: usize = 50000;

    let data: Arc<[_]> = (0..1_000_000_000).into_iter().rev().collect();
    let target = 100_000_000;

    let pool = Pool::with_thread_count(40).expect("Unable to create pool");
    let found = measure(|| pool_find(data, target, pool, chunk_size));
    println!("found: {:?}", found);
}
