use rustautogui::{errors::AutoGuiError, RustAutoGui};
use windows_capture::monitor::Monitor;

pub enum Game {
    VALORANT,
    LEAGUE
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
    pub fn load_templates(&mut self, game: Game) -> Result<(), AutoGuiError> {
        //if this fails then cd into the universal_comms_bot folder it will work then lol
        // for release il just zip the images together
        // if let Err(_) = self.rustautogui.change_ocl_device(0) {
        //     println!("Failed to use opencl! I highly recommend using it, but I will proceed.")
        // } // this might not work, so just like be aware
        if self.templates_loaded {
            return Err(AutoGuiError::ImgError(
                "The templates are already loaded!".to_string(),
            ));
        }
        match game {
            Game::VALORANT => {},
            Game::LEAGUE => {
                        self.rustautogui.store_template_from_file(
            "lock_in_images/Find Match.png",
            self.window_size,
            rustautogui::MatchMode::SegmentedOcl,
            "Find match",
        )?;
        self.rustautogui.store_template_from_file(
            "lock_in_images/Accept Match.png",
            self.window_size,
            rustautogui::MatchMode::SegmentedOcl,
            "Accept match",
        )?;
        self.rustautogui.store_template_from_file(
            "lock_in_images/Search Bar.png",
            self.window_size,
            rustautogui::MatchMode::SegmentedOcl,
            "Search bar",
        )?;
        self.rustautogui.store_template_from_file(
            "lock_in_images/Offset down for champ portrait.png",
            self.window_size,
            rustautogui::MatchMode::SegmentedOcl,
            "topjungle offset for portrait",
        )?;
        self.rustautogui.store_template_from_file(
            "lock_in_images/Lock in.png",
            self.window_size,
            rustautogui::MatchMode::SegmentedOclV2,
            "Lock in",
        )?;
        self.templates_loaded = true;
            }
        }





        Ok(())
    }
}