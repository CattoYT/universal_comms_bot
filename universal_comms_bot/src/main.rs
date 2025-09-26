use core::panic;
use opencv::highgui::{self, WINDOW_NORMAL};

mod screenshots;
use processor_shared;

fn main() {
    println!("Hello, world!");

    highgui::named_window("Demo", WINDOW_NORMAL).expect("ONO");

    #[allow(unused_variables)]
    let (recv, screenshot_controller) = screenshots::capture::spawn_screenshotting_thread();

    loop {
        let frame_data = recv.recv().unwrap();
        // println!("got something");

        let Ok(mut frame) =
            processor_shared::convert_image_data(frame_data.height, frame_data.raw_buffer)
        else {
            println!("convert failed");
            // continue;
            panic!()
        };
        if let Ok(masked_img) = processor_shared::league::enemy_map_detection::convert_to_enemy_red_map(&mut frame) {
            highgui::imshow("Demo", &frame).unwrap();
            let _ = highgui::wait_key(1);
        }
        
        else {panic!()}        // let red_map = processor_shared::league::enemy_map_detection::create_enemy_red_map(&frame);
        println!("{}", recv.len() as i32);
        
    }
}
