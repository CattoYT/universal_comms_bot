use opencv::core::{CV_8UC1, KeyPoint, Point, Point2f, Point3f, Rect, Size, Vector};
use opencv::core::{Scalar, in_range};
use opencv::features2d::{SimpleBlobDetector, SimpleBlobDetector_Params};
use opencv::{imgproc, prelude::*};

type Error = opencv::error::Error;

use crate::opencv_bullshit_colour_from_rgba;

fn find_red_enemies(img: Mat) {
    let masked_image = mask_image_for_enemies(&img);
}

pub fn mask_image_for_enemies(img: &Mat) -> Result<Mat, Error> {
    let mut result_image = Mat::new_rows_cols_with_default(
        img.rows(),
        img.cols(),
        CV_8UC1,
        Scalar::new(0., 0., 0., 255.),
    )?;
    in_range(
        img,
        &opencv_bullshit_colour_from_rgba(170, 10, 10, 255), //todo: Find colours for colourblind settings cuz i dont even use red myself lmao i stole the value from a pixelbot on github
        &opencv_bullshit_colour_from_rgba(255, 60, 60, 255),
        &mut result_image,
    )?;

    // probably run blob detection on this for more accuracy
    let mut blob_params = SimpleBlobDetector_Params::default().unwrap();
    blob_params.blob_color = 255;
    blob_params.filter_by_color = true;

    //chatgpt trying to help here
    let kernel = imgproc::get_structuring_element(
        imgproc::MORPH_ELLIPSE,
        Size::new(10, 10), // bigger kernel = more connection
        Point::new(-1, -1),
    )?;
    imgproc::dilate(
        &result_image.clone(),
        &mut result_image,
        &kernel,
        Point::new(-1, -1),
        2,
        opencv::core::BORDER_CONSTANT,
        imgproc::morphology_default_border_value()?,
    )?;
    //end of cgpt

    //todo: find out how to get a consistent blob cuz i dont want to implement yolov8 again lmao

    let mut blob_detector: opencv::core::Ptr<SimpleBlobDetector> =
        SimpleBlobDetector::create(blob_params).unwrap();
    let mut keypoints: Vector<KeyPoint> = Vector::new();

    blob_detector.detect_def(&result_image, &mut keypoints);
    let centers: Vec<Point2f> = keypoints.iter().map(|k| k.pt()).collect();
    println!("{:?}", centers);
    Ok(result_image)

    // Err(Error::new(0, "Failed to mask image"))
}
