// this is essentially starting a divergent universe run with acheron, killing the first 3 enemies, then backing out for the 50 sync points
// tested this a while back and its the fastest way to afk the point gathering

use std::sync::Arc;

use crossbeam::channel::Receiver;
use rustautogui::errors::AutoGuiError;

use crate::{autogui::RustAutoGuiHelper, screenshots::frame::FrameData};

pub fn spam_divergent_universe(consumer_recv: Receiver<Arc<FrameData>>) {
    std::thread::spawn(move || {
        let frame_data = consumer_recv.recv().unwrap();
        run_divergent_universe(frame_data)
    });
}

fn run_divergent_universe(frame_data: Arc<FrameData>) -> Result<(), AutoGuiError> {
    let mut autogui = RustAutoGuiHelper::new();

    let Ok(mut frame) =
        processor_shared::convert_image_data(frame_data.height, &frame_data.raw_buffer)
    else {
        println!("convert failed");
        panic!()
    };

    Ok(())
}
