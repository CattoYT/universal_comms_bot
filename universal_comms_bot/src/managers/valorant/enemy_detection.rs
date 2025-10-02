// because I don't want to get fucking banned, i'm only using colour detection

use std::sync::Arc;

use crossbeam::channel::Receiver;
use opencv::{core::Mat, highgui};

use crate::screenshots::frame::FrameData;


pub fn process_valorant(consumer_recv: Receiver<Arc<FrameData>>) {
    println!("Started valorant detection");
    std::thread::spawn(move || {
        loop {
            let raw_image_data = consumer_recv.recv().unwrap();

            let raw_image = match processor_shared::convert_image_data(
                raw_image_data.height,
                &raw_image_data.raw_buffer,
            ) {
                Ok(img) => img,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };

            let processed_image = processor_shared::valorant::enemy_pixel_detection::mask_image_for_enemies(&raw_image).unwrap();
            highgui::imshow("Val output", &processed_image).unwrap();

            let _ = highgui::wait_key(1);
            
        }
    });
}
