use std::sync::Arc;

use crossbeam::channel::Receiver;

use crate::screenshots::frame::FrameData;

pub fn spam_relics(consumer_recv: Receiver<Arc<FrameData>>) {
    std::thread::spawn(move || {
        let image = consumer_recv.recv().unwrap();
        let image_mat = processor_shared::convert_image_data(image.height, &image.raw_buffer);
        
    });
}
