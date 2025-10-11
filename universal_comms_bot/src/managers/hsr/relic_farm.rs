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
                    Err(e) => {
                        
                        if let autogui::RAutoGuiError::WaitPls = e {
                            println!("Waiting 1 second...");
                            sleep(Duration::from_secs(1));
                        }else {
                            println!("{e}");
                        }
                    }
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
) -> Result<(), autogui::RAutoGuiError> {
    match stage {
        0 => {
            //start challenge and enter team select
            autogui
                .move_and_click_on_template("Challenge", true)
                .unwrap(); //todo: see why this isnt clicking
            sleep(Duration::from_millis(800));
        }
        1 => {
            //enter battle

            autogui
                .click_with_pixel_check(&image, (1675, 969), (255, 255, 255), Some(5))
                .expect("no");
            sleep(Duration::from_secs(3)); //load speed dependent
        }
        2 => {
            // wait for entering battle and enable autoplay
            match RustAutoGuiHelper::check_pixel_colour(&image, (72, 21), (255, 255, 255), Some(3))
            {
                Ok(result) => {
                    if !result {
                        return Err(rustautogui::errors::AutoGuiError::OSFailure(
                            "Wrong colour found wave counter, continuing to next frame".to_string(),
                        )
                        .into());
                    }
                }
                Err(_) => {
                    return Err(rustautogui::errors::AutoGuiError::OSFailure(
                        "colour check failed, continuing to next frame".to_string(),
                    )
                    .into());
                }
            };
            if let Ok(result) =
                RustAutoGuiHelper::check_pixel_colour(&image, (1762, 64), (239, 214, 155), Some(5))
            {
                if !result {
                    autogui.move_and_click((1762, 64)).expect("fucking hell");
                }
            }
            // match autogui.click_with_pixel_check(&image, (1762, 64), (239, 214, 155), Some(5)) {
            //     Ok(_) => {}
            //     Err(e) => {
            //         println!("{e}")
            //     }
            // }
        }
        3 => {
            //wait til finish lol
            if let Ok(result) =
                RustAutoGuiHelper::check_pixel_colour(&image, (1250, 965), (227, 228, 229), Some(5))
            {
                if result {
                    autogui.move_and_click((1250, 965));
                } else {
                    return Err(autogui::RAutoGuiError::WaitPls);
                }
            }
        }
        4 => { //TODO: check the trailblaze power and then be able to select more hopefully

        }
        _ => {
            panic!("debug panic so stfu")
        }
    }

    Ok(())
}
