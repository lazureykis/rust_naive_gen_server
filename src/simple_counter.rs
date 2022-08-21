use std::sync::mpsc;
use std::thread;

pub fn start_counter() -> (mpsc::Sender<u64>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel::<u64>();

    let join_handle = thread::spawn(move || {
        let mut counter: u64 = 0;
        for msg in receiver {
            counter = counter + msg;

            println!("Counter updated: {}", &counter);
        }
    });

    (sender, join_handle)
}
