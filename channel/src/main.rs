use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender, SendError, sync_channel, SyncSender};
use std::thread;

fn main() {
    let (tx, rx) = sync_channel(100);
    let ttx = tx;
    let s = Site { rx };
    thread::spawn(move || {
        ttx.send(123);
        ttx.send(123);
        ttx.send(123);
        ttx.send(123);
    });

    s.receive()
}

#[derive(Debug)]
pub struct Site {
    rx: Receiver<i64>,
}

impl Site {
    fn receive(&self) {
        thread::spawn(|| {
            for x in &self.rx {
                println!("{}", x);
            }
        });
    }
}