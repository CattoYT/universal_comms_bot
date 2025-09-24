use opencv::{core::{Mat, Vector}, imgcodecs, Error};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn convert_image_data(height: u32, width: u32, data: Vec<u8>) -> Result<Mat, Error> {
    let a = Mat::new_rows_cols_with_data(
        height as i32, width as i32, &data
    );
    match a {
        Ok(mat) => Ok(mat.clone_pointee()),
        Err(_) => Err(Error {code: 1, message: "Failed to convert image data".to_string()})
    }
}

pub fn save_as_image(mat: Mat) -> Result<(), Error> {
    if let Ok(_) = imgcodecs::imwrite(&"gray_image_cv2.png", &mat, &Vector::new()) {
        ()
    }
    return Err(opencv::Error {code: 1, message: "Failed to save image data".to_string()})



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
