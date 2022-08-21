use std::env::args;

mod simple_counter;

fn main() {
    let command = args()
        .nth(1)
        .expect("USAGE: rust_gen_server_example simple_counter");
    match command.as_str() {
        "simple_counter" => {
            let (sender, join_handle) = simple_counter::start_counter();

            sender.send(1).unwrap();
            sender.send(5).unwrap();
            sender.send(1).unwrap();

            join_handle.join().unwrap();
        }
        _ => println!("Unknown command"),
    }
}
