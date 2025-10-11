use std::{fmt::{format, Display}, thread::sleep, time::Duration};

use opencv::core::{Mat, MatTraitConst, Vec4b};
use rustautogui::{RustAutoGui, errors::AutoGuiError};

pub enum Game {
    VALORANT,
    LEAGUE,
    HSR(HSRMode),
}

pub enum HSRMode {
    DivergentUniverse,
    CoC, // TODO: check if its the same for cavern of corrosion and material farming
}

fn get_monitor_size() -> Option<(u32, u32, u32, u32)> {
    //could find a way to make this a little cleaner
    let mut temp_gui = RustAutoGui::new(false).unwrap();

    let (x, y) = temp_gui.get_screen_size();

    Some((0, 0, x as u32, y as u32))
}

#[derive(Debug)]
pub enum RAutoGuiError {
    WrongColour,
    AutoGuiError(AutoGuiError),
    OpenCVError(opencv::Error),
    WaitPls, //todo: hold a duration probalby
}

impl From<AutoGuiError> for RAutoGuiError {
    fn from(value: AutoGuiError) -> Self {
        Self::AutoGuiError(value)
    }
}
impl From<opencv::Error> for RAutoGuiError {
    fn from(value: opencv::Error) -> Self {
        Self::OpenCVError(value)
    }
}

impl Display for RAutoGuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RAutoGuiError::WrongColour => write!(f, "The pixel was not the expected colour"),
            RAutoGuiError::AutoGuiError(e) => write!(f, "An AutoGui error occurred: {e}"),
            RAutoGuiError::OpenCVError(e) => write!(f, "An OpenCV error occurred: {e}"),
            RAutoGuiError::WaitPls => write!(f, "Possibly fixable by sleep()")
        }
        
    }
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
            self.rustautogui.left_click().expect("an");
        } else {
            self.rustautogui
                .find_stored_image_on_screen_and_move_mouse(0.6, 0.05, template)?;
            self.rustautogui.left_click().expect("an");
        }
        Ok(())
    }
    pub fn move_and_click(&mut self, coords: (u32, u32)) -> Result<(), AutoGuiError> {
        self.rustautogui.move_mouse_to_pos(coords.0, coords.1, 0.05);
        self.rustautogui.left_click().expect("an");
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
            Game::HSR(mode) => match mode {
                HSRMode::DivergentUniverse => {
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
                HSRMode::CoC => {
                    self.rustautogui.store_template_from_file(
                        "hsr_images/relics/challenge.png",
                        self.window_size,
                        match_mode,
                        "Challenge",
                    )?;
                }
            },
        }

        Ok(())
    }
    pub fn click_with_pixel_check(
        &self,
        image: &Mat,
        coords: (i32, i32),
        colour_rgb: (u8, u8, u8),
        tolerance: Option<u8>,
    ) -> Result<(), RAutoGuiError> {
        match Self::check_pixel_colour(image, coords, colour_rgb, tolerance) {
            Ok(pixel_status) => {
                if !pixel_status {
                    return Err(RAutoGuiError::WrongColour);
                }
                self.rustautogui
                    .move_mouse_to_pos(coords.0 as u32, coords.1 as u32, 0.05)
                    .expect("failed to move???");
                sleep(Duration::from_millis(60));
                self.rustautogui.left_click().expect("failed to click");
                return Ok(());
            }
            Err(e) => {
                return Err(RAutoGuiError::AutoGuiError(AutoGuiError::OSFailure(
                    format!("?? {e}").to_string(),
                )));
            }
        };
    }

    pub fn check_pixel_colour(
        image: &Mat,
        coords: (i32, i32),
        colour_rgb: (u8, u8, u8),
        tolerance: Option<u8>,
    ) -> Result<bool, opencv::Error> {
        let pixel: &Vec4b = image.at_2d(coords.1, coords.0)?;
        let colour_bgr: [u8; 4] = [colour_rgb.2, colour_rgb.1, colour_rgb.0, 255];
        println!("{:?}", pixel.0);

        if let Some(tolerance) = tolerance {
            let lower_bounds: [u8; 4] = colour_bgr
                .iter()
                .map(|x| x.saturating_sub(tolerance))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            let upper_bounds: [u8; 4] = colour_bgr
                .iter()
                .map(|x| x.saturating_add(tolerance))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            // who the fuck cares about the alpha lmao

            //ty copilot
            let within =
                (0..3).all(|i| pixel.0[i] >= lower_bounds[i] && pixel.0[i] <= upper_bounds[i]);
            return Ok(within);
        }

        if pixel.0 == colour_bgr {
            return Ok(true);
        }

        Ok(false)
    }
}
