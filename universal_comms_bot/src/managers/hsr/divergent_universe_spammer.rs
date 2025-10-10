// this is essentially starting a divergent universe run with acheron, killing the first 3 enemies, then backing out for the 50 sync points
// tested this a while back and its the fastest way to afk the point gathering
// TODO: Refactor this entire file to use my shitty tolerance
// TODO: Refactor a ton of the stages to remove loops and actually use the fucking stage system i made

use core::panic;
use std::{any::Any, sync::Arc, thread::sleep, time::Duration};

use crossbeam::channel::Receiver;
use opencv::core::{Mat, MatTraitConst, Vec4b, VecN};
use rustautogui::errors::AutoGuiError;

use crate::{autogui::RustAutoGuiHelper, screenshots::frame::FrameData};

pub fn spam_divergent_universe(consumer_recv: Receiver<Arc<FrameData>>) {
    std::thread::spawn(move || {
        let mut _du_counter = 0; //left the underscore there as a temporary "compiler stfu please"
        let mut stage = 0;
        let mut autogui = RustAutoGuiHelper::new();
        autogui
            .load_templates(crate::autogui::Game::HSR(crate::autogui::HSRMode::DivergentUniverse))
            .expect("a");
        loop {
            let frame_data = consumer_recv.recv().unwrap();
            println!("starting stage {stage}");
            match run_divergent_universe(&mut autogui, stage, frame_data) {
                Ok(_) => {
                    stage += 1;
                }
                Err(e) => println!("{e}"),
            }
            for x in 0..consumer_recv.len() {
                drop(consumer_recv.recv()) //TODO: make this much better, cuz theres no point in screenshotting if its being dropped 
            }
            if stage > 11 {
                stage = 0
            }
        }
    });
}

fn run_divergent_universe(
    autogui: &mut RustAutoGuiHelper,
    stage: u8,
    frame_data: Arc<FrameData>,
) -> Result<(), AutoGuiError> {
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
            match autogui.click_with_pixel_check(&frame, (1400, 950), (221, 192, 140), None)
            {
                Ok(_) => {
                    sleep(Duration::from_millis(1000));
                    return Ok(());
                }
                Err(_) => {
                    sleep(Duration::from_secs(1));
                    return Err(AutoGuiError::OSFailure(
                        "Waiting for you to enter the Divergent Universe home screen".to_string(),
                    ));
                }
            }
        } // cyclical extrapolation
        1 => {
            //check if previous was actually successful
            if RustAutoGuiHelper::check_pixel_colour(&frame, (1400, 950), (221, 192, 140), None).unwrap() == true {
                panic!(
                    "It's likely Star Rail blocked clicks. \nPlease rerun this application with admin perms to use the Divergent Universe bot :/"
                );
            }
            let _ = autogui.move_and_click((178, 570));
            sleep(Duration::from_millis(700));
        } //difficulty 5
        2 => {
            let _ = autogui.move_and_click((128, 609));
            sleep(Duration::from_millis(800));
        } //check acheron and start
        3 => {
            if RustAutoGuiHelper::check_pixel_colour(&frame, (1122, 970), (235, 45, 59), None).unwrap() {
                println!("Acheron confirmed, proceeding");
                // let _ = click_with_pixel_check(&autogui.rustautogui, &frame, (1688, 960), (225,225,225));
                autogui.move_and_click((1688, 960));
                let _ = autogui
                    .rustautogui
                    .loop_find_stored_image_on_screen(1.0, 30, "View Obtained Curios")
                    .expect("a"); //wait for load to finish then proceed
                println!("next stage detected");
            } else {
                todo!("find acheron")
            }
        }
        4 => {
            // get first curio
            let centre = autogui.rustautogui.get_screen_size();
            let _ = autogui.move_and_click((centre.0 as u32 / 2, centre.1 as u32 / 2));
            sleep(Duration::from_millis(100));
            let _ = &autogui.move_and_click((1708, 960)).expect("4");
            sleep(Duration::from_millis(2200));
            let _ = &autogui.move_and_click((1500, 960)).expect("4");
            sleep(Duration::from_millis(3000));
        }
        5 => {
            //boon
            let centre = autogui.rustautogui.get_screen_size();
            let _ = autogui.move_and_click((centre.0 as u32 / 2, centre.1 as u32 / 2));
            sleep(Duration::from_millis(100));
            let _ = autogui.move_and_click((1032, 982));
            sleep(Duration::from_millis(1500));
        }
        6 => {
            // get first curio
            let centre = autogui.rustautogui.get_screen_size();
            let _ = autogui.move_and_click((centre.0 as u32 / 2, centre.1 as u32 / 2));
            sleep(Duration::from_millis(100));
            let _ = &autogui.move_and_click((1708, 960)).expect("4");
            sleep(Duration::from_millis(1000));
        }
        7 => {
            //TODO: OPTIMIZE
            loop {
                let r1: bool = loop {
                    match &autogui
                        .rustautogui
                        .find_stored_image_on_screen(0.9, "View Blessings And Equations")
                        .unwrap()
                    {
                        Some(_) => {
                            // println!("VBAE");
                            let centre = autogui.rustautogui.get_screen_size();
                            let _ =
                                autogui.move_and_click((centre.0 as u32 / 2, centre.1 as u32 / 2));
                            sleep(Duration::from_millis(100));
                            let _ = &autogui.move_and_click((1708, 960)).expect("4");
                            sleep(Duration::from_millis(1000));
                            let _ = &autogui.move_and_click((1500, 960)).expect("4");
                        }
                        None => break true,
                    }
                };
                let r2: bool = loop {
                    println!("searching for ba");
                    match &autogui.rustautogui.find_stored_image_on_screen(
                        0.7,
                        "Blank area",
                    ) {
                        Ok(_) => {
                            // println!("Blank area");
                            let _ = &autogui.move_and_click((1500, 960)).expect("4");
                            
                        }
                        Err(_) => {
                            break true;
                        }
                    };
                };
                if r1 && r2 {
                    return Ok(());
                } else {
                    sleep(Duration::from_secs(1));
                }
            }
            //hopefully atp we are in game?
        }
        8 => {
            //kill the 3 starting enemies
            let _ = &autogui.rustautogui.key_down("w");
            for _ in 0..8 {
                let _ = &autogui.rustautogui.keyboard_input("e");
                sleep(Duration::from_millis(500));
            }
            let _ = &autogui.rustautogui.key_up("w");
        }
        9 => {
            //receive blessings
            loop {
                match &autogui.rustautogui.loop_find_stored_image_on_screen(
                    0.9,
                    3,
                    "View Blessings And Equations",
                ) {
                    Ok(_) => {
                        println!("VBAE");
                        let centre = autogui.rustautogui.get_screen_size();
                        let _ = autogui.move_and_click((centre.0 as u32 / 2, centre.1 as u32 / 2));
                        sleep(Duration::from_millis(100));
                        let _ = &autogui.move_and_click((1708, 960)).expect("4");
                    }
                    Err(_) => break,
                }
                sleep(Duration::from_millis(1500));
            }
        }
        10 => {
            //back out
            autogui.rustautogui.keyboard_command("escape").expect("10");

            sleep(Duration::from_millis(1000));
            let _ = autogui.move_and_click((1576, 980));
            sleep(Duration::from_millis(1500));
            let _ = autogui.move_and_click((1178, 775));
        }
        11 => {
            let coords = autogui
                .rustautogui
                .loop_find_stored_image_on_screen(1.0, 30, "R2Main Menu")
                .unwrap();
            match coords {
                Some(_) => {
                    autogui
                        .move_and_click((960, 960))
                        .expect("hope this works lol");
                }
                None => return Err(AutoGuiError::OSFailure("Couldn't exit?".to_string())),
            }
        }
        _ => {
            println!("Restarting!");
        }
    }
    Ok(())
}

#[derive(Debug)]
struct DivergentUniverseError(String);



// notes
// ok so a du run starts mostly the same as it did before so
// start with click on start in the DU menu
// check with opencv -> (1400, 950) rgb 221,192,140 x
// (178, 570) click x
// (128, 609) click x
// (1150, 969) check if its a pixel on acheron if not then probably can do template matching for acheron
//      rgb 139,76,108 at (1150, 969) if acheron is first slot x

// (1688, 960) rgb 225,225,225 x
// click centre of screen when view obtained curios is available x
// (1688, 960) click x
// click centre x
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
