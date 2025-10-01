// because I don't want to get fucking banned, i'm only using colour detection

use std::sync::Arc;

use crossbeam::channel::Receiver;
use opencv::core::Mat;

use crate::screenshots::frame::FrameData;

pub fn find_red_enemies(consumer_recv: Receiver<Arc<FrameData>>) {
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


        }
    });
}

pub fn mask_image_for_enemies(img: Mat) {
    //todo: Find colours for colourblind settings cuz i dont even use red myself lmao i stole the value from a pixelbot on github
    // # Color detection settings (HSV)
    // self.lower_color = np.array([150, 76,  123])
    // self.upper_color = np.array([160, 197, 255])
}
