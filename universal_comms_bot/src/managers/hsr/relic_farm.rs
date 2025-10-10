use core::panic;
use std::{sync::Arc, thread::sleep, time::Duration};

use crossbeam::channel::Receiver;
use opencv::core::Mat;

use crate::{
    autogui::{self, RustAutoGuiHelper},
    screenshots::frame::FrameData,
};

pub fn spam_relics(consumer_recv: Receiver<Arc<FrameData>>) {
    std::thread::spawn(move || {
        let mut stage = 0;
        let mut autogui = RustAutoGuiHelper::new();
        autogui
            .load_templates(crate::autogui::Game::HSR(autogui::HSRMode::CoC))
            .expect("a");
        loop {
            let image = consumer_recv.recv().unwrap();
            if let Ok(image_mat) =
                processor_shared::convert_image_data(image.height, &image.raw_buffer)
            {
                println!("starting stage {stage}");
                match run_relics(&mut autogui, stage, image_mat) {
                    Ok(_) => {
                        stage += 1;
                    }
                    Err(e) => println!("{e}"),
                }
            } else {
                continue;
            }

            for _ in 0..consumer_recv.len() {
                drop(consumer_recv.recv()) //TODO: make this much better, cuz theres no point in screenshotting if its being dropped 
            }
        }
    });
}

fn run_relics(
    autogui: &mut RustAutoGuiHelper,
    stage: i32,
    image: Mat,
) -> Result<(), rustautogui::errors::AutoGuiError> {

    match stage {
         0 => {
            autogui.move_and_click_on_template("Challenge", true).unwrap(); //todo: see why this isnt clicking
            sleep(Duration::from_millis(800));
         }
         1 => {
            // autogui.click_with_pixel_check(&image, (1675, 969), (242, 243, 244), Some(5)).expect("no");
         }
        _ => {
            panic!("debug panic so stfu")
        }
    }

    Ok(())

}
