use opencv::core::{AlgorithmHint, Point, Size, BORDER_CONSTANT, CV_32FC1, CV_32FC3, CV_8UC1};
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

    rectangle(
        &mut masked_image,
        opencv::core::Rect::new(1628, 699, 8, 9),
        opencv_bullshit_colour_from_rgba(0, 0, 0, 255),
        0,
        0,
        0,
    )
    .unwrap();

    Ok(masked_image)
}

pub fn convert_to_enemy_red_map(image: &mut Mat) -> Result<Mat, Error> {
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
    let mut temp = Mat::new_rows_cols_with_default(
        image.rows(),
        image.cols(),
        CV_32FC3,
        Scalar::new(0., 0., 0., 255.),
    )?;
    let mut circles = Mat::default();
    println!("a");
    hough_circles(
        &result_image,
        &mut circles,
        HOUGH_GRADIENT,
        1.5,
        60.0,
        100.0,
        20.,
        15,
        500,
    );
    println!("b");

    println!("Number of detected circles: {:?}", circles);
    //     let mut temp: Vector<Vector<Point>> = Vector::new();
    //     let kernel = imgproc::get_structuring_element(
    //     imgproc::MORPH_ELLIPSE,
    //     Size::new(5, 5),
    //     Point::new(-1, -1),
    // )?;
    //     imgproc::morphology_ex(
    //         &result_image.clone(),
    //         &mut result_image,
    //         imgproc::MORPH_CLOSE,
    //         &kernel,
    //         Point::new(-1, -1),
    //         2,
    //         BORDER_CONSTANT,
    //         Scalar::default(),
    //     );

    //     match imgproc::find_contours(
    //         &result_image,
    //         &mut temp,
    //         imgproc::RETR_CCOMP,
    //         imgproc::CHAIN_APPROX_NONE,
    //         Point::new(0, 0),
    //     ) {
    //         Ok(_) => {
    //             println!("{:?}", temp);
    //         }
    //         Err(e) => {
    //             return Err(Error {
    //                 code: 1,
    //                 message: format!("find_contours failed: {}", e),
    //             });
    //         }
    //     }

    Ok(result_image)
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::imgcodecs::IMREAD_COLOR;
    use opencv::imgcodecs::imread;

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
