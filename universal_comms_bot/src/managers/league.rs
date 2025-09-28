use std::{
    fs::File,
    io::{BufReader, Error, Read},
    sync::Arc, time::Duration,
};

use crossbeam::channel::Receiver;
use opencv::highgui;
use processor_shared::league::enemy_map_detection::Detections;
use rodio::{Decoder, Source};

use crate::screenshots::frame::FrameData;

struct LeagueState {
    enemies: Option<Vec<Enemy>>,
    last_update: Option<Vec<Enemy>>,
}
impl LeagueState {
    fn update_state(&mut self, new_state: Option<Vec<Enemy>>) -> Result<(), Error> {
        self.last_update = self.enemies.clone();
        self.enemies = new_state;

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Enemy {
    x: f32,
    y: f32,
    map_side: Option<JungleStatus>,
}

pub fn process_map_data(consumer_recv: Receiver<Arc<FrameData>>) {
    std::thread::spawn(move || {
        // probalem with this is that i want to share the image, so
        let mut current_state = LeagueState {
            enemies: None,
            last_update: None,
        };

        loop {
            // print!("{}[2J", 27 as char);
            println!("-------------------------");
            let frame_data = consumer_recv.recv().unwrap();

            let Ok(mut frame) =
                processor_shared::convert_image_data(frame_data.height, &frame_data.raw_buffer)
            else {
                println!("convert failed");
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
                    let mut enemies_vec = Vec::new();
                    for enemy in minimap_enemies.enemies {
                        println!("Enemy at {}, {}", enemy[0], enemy[1]);
                        println!("Enemy is in {:?}", check_river(vec![enemy[0], enemy[1]]));
                        enemies_vec.push(Enemy {
                            x: enemy[0],
                            y: enemy[1],
                            map_side: check_river(vec![enemy[0], enemy[1]]),
                        });
                    }
                    // current_state.

                    current_state.update_state(Some(enemies_vec)).unwrap();

                    let _ = highgui::wait_key(1);
                }
                Err(e) => {
                    panic!("{e}");
                }
            }

            let (enemies, _last) = (&current_state.enemies, &current_state.last_update);

            let current_len = current_state.enemies.as_ref().map(|v| v.len()).unwrap_or(0);
            let last_len = current_state
                .last_update
                .as_ref()
                .map(|v| v.len())
                .unwrap_or(0);

            match current_len.cmp(&last_len) {
                std::cmp::Ordering::Greater => {
                    println!("New enemy detected");
                    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                        .expect("open default audio stream");
                    let mixer = stream_handle.mixer();
                    let file = BufReader::new(
                        File::open("universal_comms_bot\\sfx\\Retreat_ping_SFX.ogg").unwrap(),
                    );

                    let sink = rodio::play(mixer, BufReader::new(file)).unwrap();
                    sink.set_volume(0.05);
                    std::thread::sleep(Duration::from_secs_f32(1.));
                }
                std::cmp::Ordering::Less => println!("Lost vision of enemy"),
                std::cmp::Ordering::Equal => println!("No suspected change"),
            }
        }
    });
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum JungleStatus {
    Topside,
    Botside,
}

fn check_river(coord: Vec<f32>) -> (Option<JungleStatus>) {
    // gonna see if i can determine if enemies are in topside or bottomside jungle
    let x = coord[0];
    let y = coord[1];

    if y < (x * -1.) + 400. {
        return Some(JungleStatus::Topside);
    } else if y > (x * -1.) + 440. {
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
