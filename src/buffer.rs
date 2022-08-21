use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};

enum Message {
    CommandAddLine(String),
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

                        if state.len() >= 10 {
                            Buffer::flush_buffer(&mut state);
                        }
                    }
                    Message::CommandStop => {
                        println!("Stopping working thread...");
                        Buffer::flush_buffer(&mut state);
                        break;
                    }
                }
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
