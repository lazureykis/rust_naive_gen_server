use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

enum Message {
    // counter consumes commands
    CommandIncrement(u64),
    CommandValue,

    // counter sends results back
    ResultValue(u64),
}

#[derive(Debug)]
pub struct Counter {
    // input channel used to update counter
    sender: Sender<Message>,

    // output channel used to read counter value
    receiver: Receiver<Message>,
}

impl Counter {
    // start counter thread.
    pub fn start() -> (Counter, JoinHandle<()>) {
        let (input_sender, input_receiver) = channel::<Message>();
        let (output_sender, output_receiver) = channel::<Message>();

        let counter = Counter {
            sender: input_sender,
            receiver: output_receiver,
        };

        let output_sender2 = output_sender.clone();

        let join_handle = thread::spawn(move || {
            let mut counter: u64 = 0;

            for msg in input_receiver {
                match msg {
                    Message::CommandIncrement(value) => {
                        counter = counter + value;
                        println!("Counter updated: {}", &counter)
                    }
                    Message::CommandValue => {
                        output_sender2.send(Message::ResultValue(counter)).unwrap()
                    }
                    _ => {}
                }
            }
        });

        (counter, join_handle)
    }

    pub fn increment(&self, value: u64) {
        self.sender.send(Message::CommandIncrement(value)).unwrap();
    }

    pub fn value(&self) -> u64 {
        self.sender.send(Message::CommandValue).unwrap();

        if let Message::ResultValue(value) = self.receiver.recv().unwrap() {
            value
        } else {
            0
        }
    }
}
