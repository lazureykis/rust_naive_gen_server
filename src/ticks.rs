use std::sync::mpsc::{channel, Sender};
use std::thread::{self, sleep, JoinHandle};
use std::time::Duration;

enum Message {
    CommandAddLine(String),
    CommandTick,
    CommandStop,
}

#[derive(Debug)]
pub struct Buffer {
    // input channel used to pass commands to a working thread
    sender: Sender<Message>,
}

impl Buffer {
    pub fn start() -> (Buffer, JoinHandle<()>) {
        let (sender, receiver) = channel::<Message>();

        let buffer = Buffer { sender };

        let join_handle = thread::spawn(move || {
            let mut state: Vec<String> = vec![];

            for msg in receiver {
                match msg {
                    Message::CommandAddLine(value) => {
                        println!("Added line: {}", &value);
                        state.push(value);
                    }
                    Message::CommandTick => {
                        println!("Tick");

                        Buffer::flush_buffer(&mut state);
                    }
                    Message::CommandStop => {
                        println!("Stopping working thread...");
                        Buffer::flush_buffer(&mut state);
                        break;
                    }
                }
            }
        });

        let sender_ticks = buffer.sender.clone();
        thread::spawn(move || {
            // tick every second.
            let tick_interval = Duration::from_secs(1);
            loop {
                sleep(tick_interval);
                sender_ticks.send(Message::CommandTick).unwrap();
            }
        });

        (buffer, join_handle)
    }

    fn flush_buffer(state: &mut Vec<String>) {
        println!("Flushing {} lines from buffer...", state.len());
        state.clear();
    }

    pub fn add_line(&self, line: String) {
        self.sender.send(Message::CommandAddLine(line)).unwrap();
    }

    pub fn stop(&self) {
        self.sender.send(Message::CommandStop).unwrap();
    }
}
