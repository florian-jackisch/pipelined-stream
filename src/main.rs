extern crate futures;
extern crate futures_cpupool;
extern crate rand;

use futures::{Future, Sink, Stream};

/// Sleep for a random time between 0 and 1 second.
fn sleep_random() {
    let sleep_time_ms = rand::random::<u8>() as f64 / 255.0 * 1000.0;
    std::thread::sleep(std::time::Duration::from_millis(sleep_time_ms as u64));
}

fn main() {
    // produce input
    let (mut tx, rx) = futures::sync::mpsc::channel(3);
    let handle = std::thread::Builder::new()
        .name("source".to_string())
        .spawn(|| {
            for i in 1..10 {
                sleep_random();
                println!("Source  produced {} (in thread \"{}\")",
                         i,
                         std::thread::current().name().unwrap());
                tx = tx.send(i).wait().unwrap();
            }
        })
        .unwrap();

    // generate a thread pool
    let pool = futures_cpupool::Builder::new()
        .name_prefix("pool-")
        .create();

    // process input
    let rx = rx.and_then(|i| {
            pool.spawn_fn(move || {
                sleep_random();
                println!("Stage 0 received {} (in thread \"{}\")",
                         i,
                         std::thread::current().name().unwrap());
                Ok(i)
            })
        })
        .and_then(|i| {
            pool.spawn_fn(move || {
                sleep_random();
                println!("Stage 1 received {} (in thread \"{}\")",
                         i,
                         std::thread::current().name().unwrap());
                Ok(i)
            })
        })
        .and_then(|i| {
            pool.spawn_fn(move || {
                sleep_random();
                println!("Stage 2 received {} (in thread \"{}\")",
                         i,
                         std::thread::current().name().unwrap());
                Ok(i)
            })
        });

    // consume stream
    for r in rx.wait() {
        r.unwrap();
    }
    handle.join().unwrap();
}
