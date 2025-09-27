use std::sync::Arc;

use crossbeam::channel::Receiver;
use opencv::highgui;

use crate::screenshots::frame::FrameData;

pub fn process_map_data(consumer_recv: Receiver<Arc<FrameData>>) {
    std::thread::spawn(move || { // probalem with this is that i want to share the image, so 
        loop {                                                       // TODO: Refactor so that recv runs in a separate thread and shoves a copy into every other thread's mouth ig
            let frame_data = consumer_recv.recv().unwrap();
            // println!("got something");

            let Ok(mut frame) =
                processor_shared::convert_image_data(frame_data.height, &frame_data.raw_buffer)
            else {
                println!("convert failed");
                // continue;
                panic!()
            };
            match processor_shared::league::enemy_map_detection::create_enemy_red_map(&mut frame) {
                Ok(new_mat) => {
                    highgui::imshow("Map Visualiser", &new_mat).unwrap();
                    let total_detected_minimap_enemies: u8 =
                    match processor_shared::league::enemy_map_detection::detect_enemies_on_redmap(
                        &new_mat,
                    ) {
                        Some(enemies) => enemies.total,
                        None => 0
                    };
                    println!("Current enemies in vision: {total_detected_minimap_enemies}");

                    let _ = highgui::wait_key(1);
                }
                Err(e) => {
                    panic!("{e}");
                }
            } // let red_map = processor_shared::league::enemy_map_detection::create_enemy_red_map(&frame);
            // println!("{}", recv.len() as i32);
        }
    });
}