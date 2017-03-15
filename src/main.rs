extern crate android_glue;
extern crate gl;
extern crate glutin;
extern crate libc;

use android_glue::{Event, MotionAction};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

enum Colour {
    Red,
    Green,
    Blue,
}

impl Colour {
    fn step(&self) -> Colour {
        match *self {
            Colour::Red => Colour::Green,
            Colour::Green => Colour::Blue,
            Colour::Blue => Colour::Red,
        }
    }
}

struct App {
    window: glutin::Window,
    colour: Colour,
    max_x: f32,
    max_y: f32,
}

impl App {
    fn new() -> App {
        let window = glutin::WindowBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 0)))
            .build()
            .unwrap();

        let (x, y) = window.get_inner_size_pixels().unwrap();

        App {
            window: window,
            colour: Colour::Red,
            max_x: x as f32,
            max_y: y as f32,
        }
    }

    fn init(&self) {
        unsafe {
            let _ = self.window.make_current();
            gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        }
    }

    fn paint_red(&self) {
        unsafe { gl::ClearColor(1.0, 0.0, 0.0, 1.0) };
    }
    fn paint_green(&self) {
        unsafe { gl::ClearColor(0.0, 1.0, 0.0, 1.0) };
    }
    fn paint_blue(&self) {
        unsafe { gl::ClearColor(0.0, 0.0, 1.0, 1.0) };
    }

    fn paint_the_rainbow(&self, x: f32, y: f32) {
        let rg: f32 = x / self.max_x;
        let br: f32 = y / self.max_y;

        unsafe { gl::ClearColor(rg, 0.0, br, 1.0) };
    }

    fn toggle_colour(&mut self) {
        match self.colour {
            Colour::Red => self.paint_red(),
            Colour::Green => self.paint_green(),
            Colour::Blue => self.paint_blue(),
        }
        self.colour = self.colour.step();
    }
    fn draw(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        let _ = self.window.swap_buffers();
    }
}


fn main() {
    android_glue::write_log("Hello, world!");

    let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();

    android_glue::add_sender(sender);
    android_glue::set_multitouch(true);


    let mut app = App::new();

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
                app.init();
                app.paint_green();
                app.draw();
            }
            Event::Stop | Event::Pause => {
                println!("Stop or Pause!!");
            }
            Event::TermWindow => {
                println!("Window terminated");
            }
            Event::EventMotion(motion) => {
                match motion.action {
                    MotionAction::Down => {
                        app.toggle_colour();
                        app.draw();
                    }
                    MotionAction::Move => {
                        app.paint_the_rainbow(motion.x, motion.y);
                        app.draw();
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        println!("Event: {:?}", event);
    }
}
