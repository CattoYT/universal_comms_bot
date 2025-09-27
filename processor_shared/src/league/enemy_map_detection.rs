use opencv::core::{AlgorithmHint, Point, Rect, Size, BORDER_CONSTANT, CV_32FC1, CV_32FC3, CV_8UC1};
use opencv::{
    core::{ALGO_HINT_DEFAULT, CV_16U, CV_64FC4, Point2d, Point2f, Scalar, Vector, in_range},
    imgproc::{
        self, HOUGH_GRADIENT, HOUGH_GRADIENT_ALT, connected_components_with_algorithm,
        hough_circles, hough_circles_def, rectangle,
    },
    prelude::*,
};

type Error = opencv::error::Error;

const fn opencv_bullshit_colour_from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Scalar {
    Scalar::new(blue as f64, green as f64, red as f64, alpha as f64)
}

pub fn convert_to_enemy_red_map(image: &Mat) -> Result<Mat, Error> {
    let image_src = image.clone();
    let mut result_image = Mat::new_rows_cols_with_default(
        image.rows(),
        image.cols(),
        CV_8UC1,
        Scalar::new(0., 0., 0., 255.),
    )?;
    in_range(
        &image_src,
        &opencv_bullshit_colour_from_rgba(210, 58, 49, 255),
        &opencv_bullshit_colour_from_rgba(236, 84, 69, 255),
        &mut result_image,
    )?; //atp masked image will only contain the white pixels of the enemy outlines
    // time to figure out logic regarding detection

    // this attempt doesnt work ngl but il leave it here for now
    // if let Ok(components) =
    //     connected_components_with_algorithm(image, &mut result_image, 8, CV_16U, -1)
    // {
    //     println!("{components}");
    // } else {
    //     return Err(Error {
    //         code: 1,
    //         message: "failed on connceted compomenmts".to_string(),
    //     });
    // }

    let cropped_image = Mat::roi(&result_image, Rect {
    x: 1498,
    y: 658,
    width: 422,
    height: 422,
}).unwrap();

    let mut circles = Mat::default();
    println!("a");
    let _ = hough_circles(
        &cropped_image,
        &mut circles,
        HOUGH_GRADIENT,
        1.5,
        100.0,
        100.0,
        20.,
        15,
        100,
    );
    println!("b");

    println!("Number of detected circles: {:?}", circles.total());

    Ok(cropped_image.clone_pointee())
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::imgcodecs::IMREAD_COLOR;
    use opencv::imgcodecs::imread;

    #[test]
    fn test_black_image() {
        let image = imread(
            "F:\\Nerd Shit\\Rust\\universal_comms_bot\\images\\TestData\\2 results.png",
            IMREAD_COLOR,
        ).unwrap();

        match convert_to_enemy_red_map(&image) {
            Ok(_) => println!("ok"),
            Err(e) => panic!("{e}"),
        }
    }
}
