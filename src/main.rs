extern crate android_glue;
extern crate glutin;


fn main() {
    android_glue::write_log("Hello, world!");

    let mut window = glutin::WindowBuilder::new().build().unwrap();

    window.set_title("Rust on Android");

    unsafe { window.make_current() };

    println!("Pixel format of the window: {:?}", window.get_pixel_format());
    
    for event in window.wait_events() {

        android_glue::write_log(&format!("{:?}", event));
        match event {
            glutin::Event::Closed => break,
            _ => (),
        }
    }


}
