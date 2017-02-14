extern crate futures;

use futures::{Future, Sink, Stream};

fn main() {
    // produce input
    let (mut tx, rx) = futures::sync::mpsc::channel(10);
    let handle = std::thread::spawn(|| {
        for i in 1..10 {
            tx = tx.send(i).wait().unwrap();
        }
    });

    let rx = rx.and_then(|i| {
        println!("{}", i);
        Ok(i)
    });

    rx.collect().wait().unwrap();

    handle.join().unwrap();
}
