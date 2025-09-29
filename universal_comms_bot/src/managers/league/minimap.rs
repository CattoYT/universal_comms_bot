use std::{
    fs::File,
    io::{BufReader, Error},
    sync::Arc,
    time::Duration,
};

use crossbeam::channel::Receiver;
use opencv::highgui;
use processor_shared::league::enemy_map_detection::Detections;

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum JungleStatus {
    Topside,
    Botside,
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

            let current_len = enemies.as_ref().map(|v| v.len()).unwrap_or(0);
            let last_len = current_state
                .last_update
                .as_ref()
                .map(|v| v.len())
                .unwrap_or(0);

            match current_len.cmp(&last_len) {
                std::cmp::Ordering::Greater => {
                    println!("New enemy detected");
                    let stream_handle =
                        rodio::OutputStreamBuilder::open_default_stream().expect("bro what");
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

fn check_river(coord: Vec<f32>) -> Option<JungleStatus> {
    let x = coord[0];
    let y = coord[1];

    //ignore my shitcode, its a lazy workaround for finding the river while making use of whatever y=mx+c bullshit i remembered from gcses
    if (y < (x * -1.) + 400.) //midlane bound
    && (y < 57.) // blueside left bound
    && (x > 199.) // redsude top bound
    && (x < 265.) //redside right bound
    && (y > 265.) //blueside bottom bound
    && (y > (x * -1.) + 155.) { //toplane (lazily done)
        return Some(JungleStatus::Topside);
    } else 
    if (y > (x * -1.) + 440.) //midlane bound
    && (x < 348.) //redside right bound
    && (y > 355.) //blueside bottom bound
    && (x > 140.) //blueside left bound
    && (y > 142.) //redside top bound
    && (y < (x * -1.) + 645.) //botlane (again, lazily done cuz idk how a half horizontal quadratic would work (i do but im lazy thanks further maths :<))
    {
        return Some(JungleStatus::Botside);
    }
    None


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
