use crossbeam::channel::{self, Sender};
use windows_capture::frame::FrameBuffer;
use std::{process::exit, thread};

mod screenshots;
use processor_shared;


fn main() {
    println!("Hello, world!");

    


    let (recv, screenshot_controller) = screenshots::capture::spawn_screenshotting_thread();

    loop {
        let frame_data = recv.recv().unwrap(); //wait for next frame from recv
        let Ok(frame) = processor_shared::convert_image_data(
            frame_data.height,
            frame_data.width,
            frame_data.raw_buffer,
        ) else {
            // e.g., eprintln!("convert failed");
            continue;
        };
        
        if let Ok(_) = processor_shared::save_as_image(frame) {
            exit(0)
        }

    }

}