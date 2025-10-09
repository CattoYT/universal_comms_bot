use std::{thread, time::Duration};

use rustautogui;

use crate::autogui::RustAutoGuiHelper;

#[derive(Debug, Clone, Copy)]
pub struct LockInError;



pub fn start_queue_lock_in(champion: &str) -> Result<(), LockInError> {
    let mut rustautogui: RustAutoGuiHelper = RustAutoGuiHelper::new();

    rustautogui
        .load_templates(crate::autogui::Game::LEAGUE)
        .expect("Failed to load templates");

    if let Err(e) = rustautogui
        .rustautogui
        .find_stored_image_on_screen_and_move_mouse(0.8, 0.0, "Find match")
    {
        panic!("{e}")
    }
    let _ = rustautogui.rustautogui.click(rustautogui::MouseClick::LEFT);

    let _ = rustautogui
        .rustautogui
        .loop_find_stored_image_on_screen_and_move_mouse(0.6, 0.1, 180, "Accept match"); // i think 3 minutes should be enough time to leave it as automatic but i can incrase ig but i need more time to test

    let _ = rustautogui.rustautogui.click(rustautogui::MouseClick::LEFT);

    let _ = lock_champion(champion, &mut rustautogui);

    Ok(())
}

fn lock_champion(champion: &str, rustautogui: &mut RustAutoGuiHelper) -> Result<(), LockInError> {
    // rtemove pub from this later
    // pub fn lock_champion(champion: &str) -> Result<(), LockInError> {
    // rtemove pub from this later

    // let mut rustautogui: RustAutoGuiHelper = RustAutoGuiHelper {
    //     window_size: Some((
    //         0,
    //         0,
    //         Monitor::primary().unwrap().width().unwrap(),
    //         Monitor::primary().unwrap().height().unwrap(),
    //     )),
    //     rustautogui: rustautogui::RustAutoGui::new(false).unwrap(),
    // };

    // rustautogui
    //     .load_templates()
    //     .expect("Failed to load templates");

    let _ = rustautogui.move_and_click_on_template("Search bar", true);
    let _ = rustautogui.rustautogui.click(rustautogui::MouseClick::LEFT);

    let _ = rustautogui.rustautogui.keyboard_input(champion);
    // figure out how to find the champion portrait possible need for text recognition :/
    // cba to do this properly so enjoy this implementation
    let possible_location = rustautogui
        .rustautogui
        .find_stored_image_on_screen(0.9, "topjungle offset for portrait");

    match possible_location {
        Ok(confirmed_location) => {
            if let Some(coords) = confirmed_location {
                let coords = coords[0];
                let _ = rustautogui
                    .rustautogui
                    .move_mouse_to_pos(coords.0 + 5, coords.1 + 40, 0.2);
            }
        }
        Err(_) => return Err(LockInError),
    }
    thread::sleep(Duration::from_secs(1));
    let _ = rustautogui.rustautogui.click(rustautogui::MouseClick::LEFT);

    let _ = rustautogui.move_and_click_on_template("Lock in", true);
    thread::sleep(Duration::from_secs(1));
    let _ = rustautogui.rustautogui.click(rustautogui::MouseClick::LEFT);
    println!("Successfully locked in");

    Ok(())
}
