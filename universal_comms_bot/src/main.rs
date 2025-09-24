use crossbeam::channel::{self, Sender};
use windows_capture::frame::{self, FrameBuffer};
use core::panic;
use std::{process::exit, thread};

mod screenshots;
use processor_shared;

use crate::screenshots::frame::FrameData;


fn main() {
    println!("Hello, world!");

    


    let (recv, screenshot_controller) = screenshots::capture::spawn_screenshotting_thread();

    // loop {
        let frame_data = recv.recv().unwrap();

        
        // let Ok(frame) = processor_shared::convert_image_data(
        //     frame_data.height,
        //     frame_data.width,
        //     frame_data.raw_buffer,
        // ) else {
        //     println!("convert failed");
        //     // continue;
        //     panic!()
        // };
        let frame = match processor_shared::convert_image_data(frame_data.height, frame_data.width, frame_data.raw_buffer)
        {
            Ok(framea) => framea,
            Err(e) => panic!("{e}")
        };
        
        if let Ok(_) = processor_shared::save_as_image(frame) {
            
            exit(0)
        }
        println!("failed");
    // }

}