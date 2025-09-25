use std::os::raw::c_void;

use opencv::{core::{Mat, MatTraitConst, Vector}, imgcodecs, imgproc, Error};
mod league;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn convert_image_data(height: u32, width: u32, data: Vec<u8>) -> Result<Mat, Error> {
    let binding = Mat::from_slice(&data).unwrap();
    let a = binding.reshape(4, height as i32);


    
    match a {
        Ok(mat) => {
            // couldnt figure out for the life of me why this didnt work properly
            let mut mat_bgra = Mat::default();
            imgproc::cvt_color(&mat, &mut mat_bgra, imgproc::COLOR_RGBA2BGRA, 0, opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT)?;
            Ok(mat_bgra)},
        Err(e) => {
            println!("{e}");
            
            Err(Error {code: 1, message: "Failed to convert image data".to_string()})}
    }

}

pub fn save_as_image(mat: Mat) -> Result<(), Error> {
    if let Ok(_) = imgcodecs::imwrite(&"gray_image_cv2.png", &mat, &Vector::new()) {
        return Ok(())
    }
    else {
            return Err(opencv::Error {code: 1, message: "Failed to save image data".to_string()})

    }
    



}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
