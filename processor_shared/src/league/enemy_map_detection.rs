use opencv::{
    core::{in_range, Scalar, CV_64FC4}, highgui, imgcodecs::imread, prelude::*
};
use opencv::core::Vec3b;
use crate::save_as_image;

type Error = opencv::error::Error;


const fn opencv_bullshit_colour_from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Scalar {
    Scalar::new(blue as f64, green as f64, red as f64, alpha as f64)
}

pub fn create_enemy_red_map(image: &Mat) -> Result<Mat, Error> {
    let mut masked_image = Mat::new_rows_cols_with_default(
        image.rows(),
        image.cols(),
        CV_64FC4,
        Scalar::new(0., 0., 22., 255.),
    )?;

    in_range(
        image,
        &opencv_bullshit_colour_from_rgba(210, 58, 49, 255),
        &opencv_bullshit_colour_from_rgba(236, 84, 69, 255),
        &mut masked_image,
    )?; //atp masked image will only contain the white pixels of the enemy outlines
    // time to figure out logic regarding detection

    
    
    Ok(masked_image)
}

pub fn convert_to_enemy_red_map(image: &mut Mat) -> Result<(), Error> {
    let image_src = image.clone();

    in_range(
        &image_src,
        &opencv_bullshit_colour_from_rgba(210, 58, 49, 255),
        &opencv_bullshit_colour_from_rgba(236, 84, 69, 255),
        image,
    )?; //atp masked image will only contain the white pixels of the enemy outlines
    // time to figure out logic regarding detection


    
    Ok(())
}


#[cfg(test)]
mod tests {
    use opencv::imgcodecs::IMREAD_COLOR;

    use super::*;

    #[test]
    fn test_black_image() {
        let image = imread(
            "F:\\Nerd Shit\\Rust\\universal_comms_bot\\images\\League of Legends Screenshot 2025.09.25 - 19.34.53.49 copy.png",
            IMREAD_COLOR,
        ).unwrap();

        match create_enemy_red_map(&image) {
            Ok(_) => println!("ok"),
            Err(e) => panic!("{e}"),
           

        }
    }
}
