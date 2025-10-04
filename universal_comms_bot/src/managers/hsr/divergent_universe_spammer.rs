// this is essentially starting a divergent universe run with acheron, killing the first 3 enemies, then backing out for the 50 sync points
// tested this a while back and its the fastest way to afk the point gathering

use std::{any::Any, sync::Arc, thread::sleep, time::Duration};

use crossbeam::channel::Receiver;
use opencv::core::{Mat, MatTraitConst, Vec4b, VecN};
use rustautogui::errors::AutoGuiError;

use crate::{autogui::RustAutoGuiHelper, screenshots::frame::FrameData};

pub fn spam_divergent_universe(consumer_recv: Receiver<Arc<FrameData>>) {
    std::thread::spawn(move || {
        let mut _du_counter = 0; //left the underscore there as a temporary "compiler stfu please"
        let mut stage = 0;
        // loop {
        let frame_data = consumer_recv.recv().unwrap();

        match run_divergent_universe(stage, frame_data) {
            Ok(_) => {
                stage += 1;
                consumer_recv.iter(); //flush the queue
            }
            Err(e) => println!("{e}"),
        }

        // }
    });
}

fn run_divergent_universe(stage: u8, frame_data: Arc<FrameData>) -> Result<(), AutoGuiError> {
    let mut autogui = RustAutoGuiHelper::new();

    let Ok(mut frame) =
        processor_shared::convert_image_data(frame_data.height, &frame_data.raw_buffer)
    else {
        println!("convert failed");
        panic!()
    };

    // let pixel: &Vec4b = frame.at_2d(950, 1400).unwrap();
    // println!("{:?}", pixel);
    // println!("{:?}", pixel.0.type_id());
    match stage {
        0 => {
            //start divergent run
            let _ =
                click_with_pixel_check(&autogui.rustautogui, &frame, (1400, 950), (221, 192, 140));
            sleep(Duration::from_millis(2000));
            if check_pixel_colour(&frame, (1400, 950), (221, 192, 140)).unwrap() == true {
                panic!(
                    "It's likely Star Rail blocked clicks. \nPlease rerun this application with admin perms to use the Divergent Universe bot :/"
                );
            }
        } // cyclical extrapolation
        1 => {
            let _ = autogui.move_and_click((178, 570));
            sleep(Duration::from_millis(1500));
        } //difficulty 5
        2 => {
            let _ = autogui.move_and_click((128, 609));
            sleep(Duration::from_millis(300));
        }
        _ => {
            println!("How did you get here");
            return Err(AutoGuiError::OSFailure("How".to_string()));
        }
    }

    Ok(())
}

struct DivergentUniverseError(String);

fn click_with_pixel_check(
    gui: &rustautogui::RustAutoGui,
    image: &Mat,
    coords: (i32, i32),
    colour_rgb: (u8, u8, u8),
) -> Result<(), DivergentUniverseError> {
    if check_pixel_colour(image, coords, colour_rgb).unwrap_or(false) {
        gui.move_mouse_to_pos(coords.0 as u32, coords.1 as u32, 0.05)
            .expect("failed to move???");
        sleep(Duration::from_millis(60));
        gui.left_click().expect("failed to click");
    }
    Err(DivergentUniverseError("Couldn't locate pixel".to_string()))
}

fn check_pixel_colour(
    image: &Mat,
    coords: (i32, i32),
    colour_rgb: (u8, u8, u8),
) -> Result<bool, opencv::Error> {
    let pixel: &Vec4b = image.at_2d(coords.1, coords.0)?;
    let colour_bgr: [u8; 4] = [colour_rgb.2, colour_rgb.1, colour_rgb.0, 255];
    if pixel.0 == colour_bgr {
        return Ok(true);
    }

    Ok(false)
}

// notes
// ok so a du run starts mostly the same as it did before so
// start with click on start in the DU menu
// check with opencv -> (1400, 950) rgb 221,192,140 x
// (178, 570) click x
// (128, 609) click x
// (1150, 969) check if its a pixel on acheron if not then probably can do template matching for acheron
//      rgb 103,74,156 at (1150, 969) if acheron is first slot

// (1688, 960) rgb 225,225,225
// click centre of screen when view obtained curios is available
// (1688, 960) click
// click centre
// (1032, 982 ) rgb 234, 233, 234

//tb blessing
// click centre
// (1032, 982 ) rgb 234, 233, 234

//blessings obtained
// (1032, 982 ) click

// after loading press e 8 times

//clear first 3/4 blessings
// click centre of screen
// (1688, 960) click
// click centre of screen
// (1688, 960) click
// click centre of screen
// (1688, 960) click

// hold alt
// (65,83) click

//(1576,980) click
//(1178,775) click

// (960, 980) click
