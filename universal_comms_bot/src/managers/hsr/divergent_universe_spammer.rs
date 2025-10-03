// this is essentially starting a divergent universe run with acheron, killing the first 3 enemies, then backing out for the 50 sync points
// tested this a while back and its the fastest way to afk the point gathering

use std::sync::Arc;

use crossbeam::channel::Receiver;
use rustautogui::errors::AutoGuiError;

use crate::{autogui::RustAutoGuiHelper, screenshots::frame::FrameData};

pub fn spam_divergent_universe(consumer_recv: Receiver<Arc<FrameData>>) {
    std::thread::spawn(move || {
        let mut _du_counter = 0; //left the underscore there as a temporary "compiler stfu please"
        loop {
            let frame_data = consumer_recv.recv().unwrap();
            match run_divergent_universe(frame_data) {
                Ok(_) => {
                    println!("Successfully completed DU run");
                    _du_counter += 1;
                }
                Err(e) => println!("{e}"),
            }
        }
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

// notes
// ok so a du run starts mostly the same as it did before so
// start with click on start in the DU menu
// check with opencv -> (1400, 950) rgb 221,192,140   
// (178, 570) click
// (128, 609) click
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
// click centre of screene
// (1688, 960) click

// hold alt
// (65,83) click

//(1576,980) click
//(1178,775) click

// (960, 980) click