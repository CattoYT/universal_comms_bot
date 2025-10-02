// because I don't want to get fucking banned, i'm only using colour detection

use std::{sync::Arc, thread::sleep, time::{Duration, Instant}};

use crossbeam::channel::Receiver;
use opencv::{core::Mat, highgui};
use rustautogui::errors::AutoGuiError;

use crate::{
    autogui::{self, RustAutoGuiHelper},
    screenshots::frame::FrameData,
};

pub fn process_valorant(consumer_recv: Receiver<Arc<FrameData>>) {
    println!("Started valorant detection");
    std::thread::spawn(move || {
        let mut chat_cooldown_timestamp = Instant::now();
        chat_cooldown_timestamp = chat_cooldown_timestamp.checked_sub(Duration::from_secs(30)).unwrap(); //allow instant chat
        let chat_cooldown = Duration::from_secs(30);
        loop {
            let mut autogui = RustAutoGuiHelper::new();
            autogui
                .load_templates(crate::autogui::Game::VALORANT)
                .expect("Failed to load Valorant templates");

            let raw_image_data = consumer_recv.recv().unwrap();

            let raw_image = match processor_shared::convert_image_data(
                raw_image_data.height,
                &raw_image_data.raw_buffer,
            ) {
                Ok(img) => img,
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };

            if let Ok(enemies) =
                processor_shared::valorant::enemy_pixel_detection::find_yellow_enemies(&raw_image)
            {
                if enemies.is_none() {
                    continue;
                }

                
                if chat_cooldown_timestamp.elapsed() > chat_cooldown {
                    println!("Attempted to type");

                    if let Ok(new_cooldown) = type_warning(&mut autogui) {
                        chat_cooldown_timestamp = new_cooldown;
                    } else {
                        println!("Failed to type");
                    }
                }

                //this might be vanguard protected cuz yay
            } else {
                panic!();
            }

            // might re-add later, i dont actually see a point to this one ngl
            // highgui::imshow("Val output", &processed_image).unwrap();

            // let _ = highgui::wait_key(1);
        }
    });
}

fn type_warning(autogui: &mut RustAutoGuiHelper) -> Result<Instant, AutoGuiError> {
    let message = "one guy here";

    autogui.rustautogui.keyboard_command("return")?;
    sleep(Duration::from_millis(50));

    for char in message.chars() {
        autogui.rustautogui.key_down(&char.to_string())?;
        autogui.rustautogui.key_up(&char.to_string())?;
            sleep(Duration::from_millis(1));

    }

    autogui.rustautogui.keyboard_command("return")?;

    Ok(Instant::now())
}
