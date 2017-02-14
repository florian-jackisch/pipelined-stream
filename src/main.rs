extern crate futures;
extern crate futures_cpupool;

use futures::{Future, Sink, Stream};

fn main() {
    // produce input
    let (mut tx, rx) = futures::sync::mpsc::channel(10);
    let handle = std::thread::spawn(|| {
        for i in 1..10 {
            tx = tx.send(i).wait().unwrap();
        }
    });

    // generate a thread pool
    let pool = futures_cpupool::CpuPool::new_num_cpus();

    // process input
    let rx = rx.and_then(|i| {
        pool.spawn_fn(move || {
            println!("{}", i);
            Ok(i)
        })
    });

    // consume stream
    rx.collect().wait().unwrap();

    handle.join().unwrap();
}
