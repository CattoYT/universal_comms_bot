use rustautogui::{RustAutoGui, errors::AutoGuiError};
use windows_capture::monitor::Monitor;

#[derive(Debug, Clone, Copy)]
pub struct LockInError;

struct RustAutoGuiHelper {
    pub window_size: Option<(u32, u32, u32, u32)>,
    pub rustautogui: RustAutoGui,
    templates_loaded: bool,
}
impl Default for RustAutoGuiHelper {
    fn default() -> Self {
        RustAutoGuiHelper::new()
    }
}
impl RustAutoGuiHelper {
    pub fn new() -> Self {
        let window_size = Some(
            Monitor::primary()
                .map(|m| (0, 0, m.width().unwrap_or(0), m.height().unwrap_or(0)))
                .unwrap(),
        );

        RustAutoGuiHelper {
            window_size,
            rustautogui: rustautogui::RustAutoGui::new(false).unwrap(),
            templates_loaded: false,
        }
    }
    fn move_and_click_on_template(
        &mut self,
        template: &str,
        do_loop: bool,
    ) -> Result<(), AutoGuiError> {
        if do_loop {
            self.rustautogui
                .loop_find_stored_image_on_screen_and_move_mouse(0.8, 0.05, 180, template)?;
        } else {
            self.rustautogui
                .find_stored_image_on_screen_and_move_mouse(0.8, 0.05, template)?;
        }
        Ok(())
    }
    fn load_templates(&mut self) -> Result<(), AutoGuiError> {
        //if this fails then cd into the universal_comms_bot folder it will work then lol
        // for release il just zip the images together
        if let Err(_) = self.rustautogui.change_ocl_device(0) {
            println!("Failed to use opencl! I highly recommend using it, but I will proceed.")
        } // this might not work, so just like be aware

        if self.templates_loaded {
            return Err(AutoGuiError::ImgError(
                "The templates are already loaded!".to_string(),
            ));
        }

        self.rustautogui.store_template_from_file(
            "lock_in_images/Find Match.png",
            self.window_size,
            rustautogui::MatchMode::Segmented,
            "Find match",
        )?;
        self.rustautogui.store_template_from_file(
            "lock_in_images/Accept Match.png",
            self.window_size,
            rustautogui::MatchMode::Segmented,
            "Accept match",
        )?;
        self.rustautogui.store_template_from_file(
            "lock_in_images/Search Bar.png",
            self.window_size,
            rustautogui::MatchMode::Segmented,
            "Search bar",
        )?;
        self.rustautogui.store_template_from_file(
            "lock_in_images/Offset down for champ portrait.png",
            self.window_size,
            rustautogui::MatchMode::Segmented,
            "topjungle offset for portrait",
        )?;
        self.rustautogui.store_template_from_file(
            "lock_in_images/Lock in.png",
            self.window_size,
            rustautogui::MatchMode::Segmented,
            "Lock in",
        )?;
        self.templates_loaded = true;

        Ok(())
    }
}

pub fn start_queue_lock_in(champion: &str) -> Result<(), LockInError> {
    let mut rustautogui: RustAutoGuiHelper = RustAutoGuiHelper::new();

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
        .loop_find_stored_image_on_screen_and_move_mouse(0.8, 0.1, 180, "Accept match"); // i think 3 minutes should be enough time to leave it as automatic but i can incrase ig but i need more time to test

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
                    .move_mouse_to_pos(coords.0 + 5, coords.1 + 40, 0.1);
            }
        }
        Err(_) => return Err(LockInError),
    }
    let _ = rustautogui.rustautogui.click(rustautogui::MouseClick::LEFT);

    let _ = rustautogui.move_and_click_on_template("Lock in", true);

    println!("Successfully locked in");



    Ok(())
}
