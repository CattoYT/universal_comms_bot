use std::sync::Arc;

use crossbeam::channel::Receiver;
use opencv::highgui;
use processor_shared::league::enemy_map_detection::Detections;

use crate::screenshots::frame::FrameData;

pub fn process_map_data(consumer_recv: Receiver<Arc<FrameData>>) {
    std::thread::spawn(move || {
        // probalem with this is that i want to share the image, so
        loop {
            // TODO: Refactor so that recv runs in a separate thread and shoves a copy into every other thread's mouth ig
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
                    let minimap_enemies: Detections =
                    match processor_shared::league::enemy_map_detection::detect_enemies_on_redmap(
                        &new_mat,
                    ) {
                        Some(enemies) => enemies,
                        None => Detections::from_empty(),
                    };
                    println!("Current enemies in vision: {}", minimap_enemies.total);
                    for enemy in minimap_enemies.enemies {
                        println!("Enemy at {}, {}", enemy[0], enemy[1]);
                        println!("Enemy is in {:?}", check_river(vec![enemy[0], enemy[1]]));
                    }

                    print!("{}[2J", 27 as char);
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

#[derive(Debug)]
enum JungleStatus {
    Topside,
    Botside,
}

fn check_river(coord: Vec::<f32>) -> (Option<JungleStatus>) {
    // gonna see if i can determine if enemies are in topside or bottomside jungle
    let x= coord[0];
    let y = coord[1];

    if y < (x*-1.) + 400. {
        return Some(JungleStatus::Topside);
    } else if y > (x*-1.) + 440.{
        return Some(JungleStatus::Botside);
    }
    None
    
    // current plan:
    // use fucking y=mx+c omg how is this actually a valid use for it
    // None
    
}

#[cfg(test)]
mod tests {

    use super::*;
    use opencv::imgcodecs::IMREAD_COLOR;
    use opencv::imgcodecs::imread;
    use processor_shared::league::enemy_map_detection::create_enemy_red_map;
    use processor_shared::league::enemy_map_detection::detect_enemies_on_redmap;

    #[test]
    fn test_jungle_enemies() {
        let image = imread(
            "F:\\Nerd Shit\\Rust\\universal_comms_bot\\images\\TestData\\2 results.png",
            IMREAD_COLOR,
        )
        .unwrap();

        match create_enemy_red_map(&image) {
            Ok(img) => match detect_enemies_on_redmap(&img) {
                Some(count) => {
                    for enemy in count.enemies.iter() {
                        let x = enemy[0];
                        let y = enemy[1];
                        let radius = enemy[2];
                        println!("Enemy at {x}, {y}");
                        println!("Enemy is in {:?}", check_river(vec![x, y]));
                    }
                }
                None => panic!("Didnt get 2"),
            },
            Err(e) => panic!("{e}"),
        }
    }
}
