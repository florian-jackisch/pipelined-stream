extern crate futures;
extern crate futures_cpupool;
extern crate rand;

use futures::{Future, Sink, Stream};

/// Sleep for a random time between 0 and 1 second.
fn sleep_random() {
    let sleep_time_ms = rand::random::<u8>() as f64 / 255.0 * 1000.0;
    std::thread::sleep(std::time::Duration::from_millis(sleep_time_ms as u64));
}

/// Sleep for a random time, then print an integer.
fn sleep_and_print(stage: i32, i: i32) -> Result<i32, ()> {
    sleep_random();
    println!("Stage {} received {} (in thread \"{}\")",
             stage,
             i,
             std::thread::current().name().unwrap());
    Ok(i)
}

fn main() {
    // produce input
    let (mut tx, rx) = futures::sync::mpsc::channel(3);
    let handle = std::thread::Builder::new()
        .name("source".to_string())
        .spawn(|| {
            for i in 0..10 {
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
    let rx = rx.map(|i| pool.spawn_fn(move || sleep_and_print(0, i)))
        .buffered(10)
        .map(|i| pool.spawn_fn(move || sleep_and_print(1, i)))
        .buffered(10)
        .map(|i| pool.spawn_fn(move || sleep_and_print(2, i)))
        .buffered(10);

    // consume stream
    for r in rx.wait() {
        r.unwrap();
    }
    handle.join().unwrap();
}
