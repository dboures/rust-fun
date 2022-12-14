extern crate crossbeam_channel;
extern crate threadpool;

use std::thread::sleep;
use std::time::Duration;

use crossbeam_channel as channel;
use threadpool::ThreadPool;

fn work(tx: channel::Sender<usize>, task: usize) {
    // Simulate some expensive work that needs to be done.
    // This will sleep for one second.
    sleep(Duration::new(1, 0));

    tx.send(task);
}

fn main() {
    const TASKS: usize = 100;

    let (tx, rx) = channel::unbounded();
    let pool = ThreadPool::new(4);

    // Create 100 superficial units of work and let the thread pool execute them.
    for i in 0..TASKS {
        // By calling clone here, we can share our sending half with each `work` function
        // that is called.
        let tx = tx.clone();
        pool.execute(move || {
            work(tx, i);
        });
    }

    // We are done with our sending half so we can explicitly drop it here.
    drop(tx);

    for i in rx {
        if i % 5 == 0 {
            println!("done with {}% of the work", i);
        }
    }

    println!("done with all of the work");
}