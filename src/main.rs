extern crate android_glue;
extern crate gl;
extern crate glutin;
extern crate libc;

use android_glue::{Event, MotionAction};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};

fn do_ui() {
    let window = glutin::WindowBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 0)))
        .build()
        .unwrap();

    unsafe {
        window.make_current();
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
    }


    loop {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        window.swap_buffers();
        thread::sleep(time::Duration::from_millis(10));
    }


}



fn main() {
    android_glue::write_log("Hello, world!");

    let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();

    android_glue::add_sender(sender);

    loop {
        let event = match receiver.recv() {
            Ok(e) => e,
            Err(_) => continue,
        };



        match event {
            Event::Start => {
                println!("Starting app!!");
            }
            Event::InitWindow => {
                thread::spawn(do_ui);
            }
            Event::Stop | Event::Pause => {
                println!("Stop or Pause!!");
            }
            Event::TermWindow => {
                println!("Window terminated");
            }
            Event::EventMotion(motion) => {
                match motion.action {
                    MotionAction::Up =>  {
                        
                    },
                    _ => {}
                }
            }
            _ => {}
        }

        println!("Event: {:?}", event);
    }


}
