use std::env::args;
use std::thread::sleep;
use std::time::Duration;

mod buffer;
mod counter;
mod simple_counter;
mod stop_me;
mod ticks;

fn main() {
    if let Some(command) = args().nth(1) {
        match command.as_str() {
            "simple_counter" => {
                let (sender, _join_handle) = simple_counter::start_counter();

                sender.send(1).unwrap();
                sender.send(5).unwrap();
                sender.send(1).unwrap();

                sleep(Duration::from_millis(100));
            }
            "counter" => {
                let (counter, _join_handle) = counter::Counter::start();

                counter.increment(1);
                counter.increment(5);
                counter.increment(1);

                let current_value = counter.value();
                println!("Current counter value is {}", &current_value);

                sleep(Duration::from_millis(100));
            }
            "stop_me" => {
                let (counter, join_handle) = stop_me::Counter::start();

                counter.increment(1);
                counter.increment(2);

                let current_value = counter.value();
                println!("Current counter value is {}", &current_value);

                counter.stop();

                join_handle.join().unwrap();
            }
            "buffer" => {
                let (buffer, join_handle) = buffer::Buffer::start();

                for i in 1..26 {
                    let line = format!("Line {}", i);
                    buffer.add_line(line);
                }

                buffer.stop();

                join_handle.join().unwrap();
            }
            "ticks" => {
                let (buffer, join_handle) = ticks::Buffer::start();

                for i in 1..11 {
                    let line = format!("Line {}", i);
                    buffer.add_line(line);
                    sleep(Duration::from_millis(240));
                }

                buffer.stop();

                join_handle.join().unwrap();
            }
            _ => println!("USAGE: cargo run [simple_counter | counter | stop_me | buffer | ticks]"),
        }
    } else {
        println!("USAGE: cargo run [simple_counter | counter | stop_me | buffer | ticks]");
        return;
    }
}
