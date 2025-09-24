use crossbeam::channel::{self, Sender};
use windows_capture::frame::FrameBuffer;
use std::thread;

mod screenshots;



fn main() {
    println!("Hello, world!");

    


    let (recv, screenshot_controller) = screenshots::capture::spawn_screenshotting_thread();

    loop {
        let frame_data = recv.recv().unwrap();
      
    }

}