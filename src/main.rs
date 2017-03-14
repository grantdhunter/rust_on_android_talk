extern crate android_glue;


use android_glue::{write_log, Event, add_sender};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;


fn main() {
    write_log("Hello, world!");

    let (sender, receiver) : (Sender<Event>, Receiver<Event>) = mpsc::channel();

    add_sender(sender);

    thread::spawn(move || {

        loop{
            let event = receiver.recv();
            println!("Event: {:?}", event);
        }
        
    });


}
