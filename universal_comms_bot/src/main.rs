use crossbeam::channel::{self, Sender};
use opencv::highgui::{self, WINDOW_NORMAL};
use windows_capture::frame::{self, FrameBuffer};
use core::panic;
use std::{process::exit, thread};

mod screenshots;
use processor_shared;

use crate::screenshots::frame::FrameData;


fn main() {
    println!("Hello, world!");

    highgui::named_window("Demo", WINDOW_NORMAL);


    let (recv, screenshot_controller) = screenshots::capture::spawn_screenshotting_thread();

    loop {
        let frame_data = recv.recv().unwrap();
        println!("got something");
        
        let Ok(mut frame) = processor_shared::convert_image_data(
            frame_data.height,
            frame_data.width,
            frame_data.raw_buffer,
        ) else {
            println!("convert failed");
            // continue;
            panic!()
        };
        // let red_map = processor_shared::league::enemy_map_detection::create_enemy_red_map(&frame);
        highgui::imshow("Demo", &frame).unwrap();
        highgui::wait_key(1);
    }

}