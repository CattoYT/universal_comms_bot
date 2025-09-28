
use rustautogui::{RustAutoGui, errors::AutoGuiError};
use windows_capture::monitor::Monitor;

#[derive(Debug, Clone, Copy)]
pub struct LockInError;

struct RustAutoGuiHelper {
    pub window_size: Option<(u32, u32, u32, u32)>,
    pub rustautogui: RustAutoGui,
}

impl RustAutoGuiHelper {
    fn move_and_click_on_template(&mut self, template: &str) -> Result<(), AutoGuiError> {
        self.rustautogui
            .find_stored_image_on_screen_and_move_mouse(0.8, 0.05, template)?;
        Ok(())
    }
    fn load_templates(&mut self) -> Result<(), AutoGuiError> {
        self.rustautogui.store_template_from_file(
            "../universal_comms_bot/lock_in_images/Find Match.png",
            self.window_size,
            rustautogui::MatchMode::Segmented,
            "Find match",
        )?;
        self.rustautogui.store_template_from_file(
            "../universal_comms_bot/lock_in_images/Accept Match.png",
            self.window_size,
            rustautogui::MatchMode::Segmented,
            "Accept match",
        )?;

        self.rustautogui.store_template_from_file(
            "../universal_comms_bot/lock_in_images/Search Bar.png",
            self.window_size,
            rustautogui::MatchMode::Segmented,
            "Search bar",
        )?;
        self.rustautogui.store_template_from_file(
            "../universal_comms_bot/lock_in_images/Offset down for champ portrait.png",
            self.window_size,
            rustautogui::MatchMode::Segmented,
            "topjungle offset for portrait",
        )?;
        Ok(())
    }
}

pub fn start_queue_lock_in(champion: &str) -> Result<(), LockInError> {
    let mut rustautogui: RustAutoGuiHelper = RustAutoGuiHelper {
        window_size: Some((
            0,
            0,
            Monitor::primary().unwrap().width().unwrap(),
            Monitor::primary().unwrap().height().unwrap(),
        )),
        rustautogui: rustautogui::RustAutoGui::new(false).unwrap(),
    };

    rustautogui
        .load_templates()
        .expect("Failed to load templates");

    if let Err(_) = rustautogui
        .rustautogui
        .find_stored_image_on_screen_and_move_mouse(0.8, 0.0, "Find match")
    {
        panic!("Failed to find image and move mouse")
    }
    let _ = rustautogui.rustautogui.click(rustautogui::MouseClick::LEFT);

    let _ = rustautogui
        .rustautogui
        .find_stored_image_on_screen_and_move_mouse(0.8, 0.1, "Accept match");

    let _ = rustautogui.rustautogui.click(rustautogui::MouseClick::LEFT);

    let _ = lock_champion(champion, &mut rustautogui);

    Ok(())
}

fn lock_champion(champion: &str, rustautogui: &mut RustAutoGuiHelper) -> Result<(), LockInError> {


    

    let _ = rustautogui.move_and_click_on_template("Search bar");
    let _ = rustautogui.rustautogui.click(rustautogui::MouseClick::LEFT);

    let _ = rustautogui.rustautogui.keyboard_input(champion);
    // figure out how to find the champion portrait possible need for text recognition :/
    // cba to do this properly so enjoy this implementation
    let possible_location = rustautogui.rustautogui.find_stored_image_on_screen(0.9, "topjungle offset for portrait");
    match possible_location {
        Ok(confirmed_location) => {
            if let Some(coords) = confirmed_location {
                let coords = coords[0];
                rustautogui.rustautogui.move_mouse_to_pos(coords.0, coords.1-30, 0.1); //TODO: find the correct offset for Y and also test like this entire file
            }
        },
        Err(_) => return Err(LockInError),
    
    }
    
    Ok(())
}
