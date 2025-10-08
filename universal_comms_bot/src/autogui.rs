use rustautogui::{RustAutoGui, errors::AutoGuiError};

pub enum Game {
    VALORANT,
    LEAGUE,
    HSR,
}

fn get_monitor_size() -> Option<(u32, u32, u32, u32)> {
    //could find a way to make this a little cleaner
    let mut temp_gui = RustAutoGui::new(false).unwrap();

    let (x, y) = temp_gui.get_screen_size();

    Some((0, 0, x as u32, y as u32))
}

pub struct RustAutoGuiHelper {
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
        let window_size = get_monitor_size();

        RustAutoGuiHelper {
            window_size,
            rustautogui: rustautogui::RustAutoGui::new(false).unwrap(),
            templates_loaded: false,
        }
    }
    pub fn move_and_click_on_template(
        &mut self,
        template: &str,
        do_loop: bool,
    ) -> Result<(), AutoGuiError> {
        if do_loop {
            self.rustautogui
                .loop_find_stored_image_on_screen_and_move_mouse(0.6, 0.05, 180, template)?;
        } else {
            self.rustautogui
                .find_stored_image_on_screen_and_move_mouse(0.6, 0.05, template)?;
        }
        Ok(())
    }
    pub fn move_and_click(&mut self, coords: (u32, u32)) -> Result<(), AutoGuiError> {
        self.rustautogui.move_mouse_to_pos(coords.0, coords.1, 0.05);
        self.rustautogui.left_click();
        Ok(())
    }
    pub fn load_templates(&mut self, game: Game) -> Result<(), AutoGuiError> {
        //if this fails then cd into the universal_comms_bot folder it will work then lol
        // for release il just zip the images together
        let mut match_mode = rustautogui::MatchMode::SegmentedOcl;
        if let Err(_) = self.rustautogui.change_ocl_device(0) {
            println!("Failed to use opencl! I highly recommend using it, but I will proceed.");
            match_mode = rustautogui::MatchMode::Segmented;
        } // this might not work, so just like be aware
        if self.templates_loaded {
            return Err(AutoGuiError::ImgError(
                "The templates are already loaded!".to_string(),
            ));
        }
        match game {
            Game::VALORANT => {}
            Game::LEAGUE => {
                self.rustautogui.store_template_from_file(
                    "lock_in_images/Find Match.png",
                    self.window_size,
                    match_mode.clone(),
                    "Find match",
                )?;
                self.rustautogui.store_template_from_file(
                    "lock_in_images/Accept Match.png",
                    self.window_size,
                    match_mode.clone(),
                    "Accept match",
                )?;
                self.rustautogui.store_template_from_file(
                    "lock_in_images/Search Bar.png",
                    self.window_size,
                    match_mode.clone(),
                    "Search bar",
                )?;
                self.rustautogui.store_template_from_file(
                    "lock_in_images/Offset down for champ portrait.png",
                    self.window_size,
                    match_mode.clone(),
                    "topjungle offset for portrait",
                )?;
                self.rustautogui.store_template_from_file(
                    "lock_in_images/Lock in.png",
                    self.window_size,
                    match_mode.clone(),
                    "Lock in",
                )?;
                self.templates_loaded = true;
            }
            Game::HSR => {
                self.rustautogui.store_template_from_file(
                    "hsr_images/du/view obtained curios.png",
                    self.window_size,
                    match_mode.clone(),
                    "View Obtained Curios",
                )?;
                self.rustautogui.store_template_from_file(
                    "hsr_images/du/View blessings and equations.png",
                    self.window_size,
                    match_mode.clone(),
                    "View Blessings And Equations",
                )?;
                self.rustautogui.store_template_from_file(
                    "hsr_images/du/Blank area.png",
                    self.window_size,
                    match_mode.clone(),
                    "Blank area",
                )?;
                self.rustautogui.store_template_from_file(
                    "hsr_images/du/return to main menu.png",
                    self.window_size,
                    match_mode.clone(),
                    "R2Main Menu",
                )?;
                self.rustautogui.store_template_from_file(
                    "hsr_images/du/return to main menu.png",
                    self.window_size,
                    match_mode,
                    "R2Main Menu",
                )?;
            }
        }

        Ok(())
    }
}
