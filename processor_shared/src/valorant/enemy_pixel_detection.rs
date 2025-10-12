use opencv::core::{CV_8UC1, KeyPoint, Point, Point2f, Point3f, Rect, Size, Vector};
use opencv::core::{Scalar, in_range};
use opencv::features2d::{SimpleBlobDetector, SimpleBlobDetector_Params};
use opencv::{highgui, imgproc, prelude::*};

type Error = opencv::error::Error;

use crate::opencv_bullshit_colour_from_rgba;

pub fn find_yellow_enemies(img: &Mat) -> Result<Option<()>, Error> {
    let mut masked_image = mask_image_for_enemies(img).unwrap();
    // probably run blob detection on this for more accuracy
    let mut blob_params = SimpleBlobDetector_Params::default().unwrap();
    blob_params.blob_color = 255;
    blob_params.filter_by_color = true;
    blob_params.min_area = 3.;

    //chatgpt trying to help here
    let kernel = imgproc::get_structuring_element(
        imgproc::MORPH_ELLIPSE,
        Size::new(10, 10), // bigger kernel = more connection
        Point::new(-1, -1),
    )?;
    imgproc::dilate(
        &masked_image.clone(),
        &mut masked_image,
        &kernel,
        Point::new(-1, -1),
        2,
        opencv::core::BORDER_CONSTANT,
        imgproc::morphology_default_border_value()?,
    )?;
    //end of cgpt

    let mut blob_detector: opencv::core::Ptr<SimpleBlobDetector> =
        SimpleBlobDetector::create(blob_params).unwrap();
    let mut keypoints: Vector<KeyPoint> = Vector::new();

    let _ = blob_detector.detect_def(&masked_image, &mut keypoints);
    let centers: Vec<Point2f> = keypoints.iter().map(|k| k.pt()).collect();
    if centers.len() > 0 {
        println!("There is most definitely an enemy here"); // I am really bad at opencv, so without machine learning, this is the best, reliable output i can reasonably give
        return Ok(Some(()));
    }

    return Ok(None);
}

pub fn mask_image_for_enemies(img: &Mat) -> Result<Mat, Error> {
    let mut result_image = Mat::new_rows_cols_with_default(
        img.rows(),
        img.cols(),
        CV_8UC1,
        Scalar::new(0., 0., 0., 255.),
    )?;
    //todo: config file?
    //yellow
    // in_range(
    //     img,
    //     &opencv_bullshit_colour_from_rgba(240, 240, 40, 255), //todo: fix the colour values and make em a little more accurate
    //     &opencv_bullshit_colour_from_rgba(252, 252, 80, 255), //cba now its probably fine
    //     &mut result_image,
    // )?;
    //purple
    in_range(
        img,
        &opencv_bullshit_colour_from_rgba(245, 80, 240, 255), //todo: fix the colour values and make em a little more accurate
        &opencv_bullshit_colour_from_rgba(255, 150, 255, 255), //cba now its probably fine
        &mut result_image,
    )?;
    let _ = highgui::imshow("yoo", &result_image);
    let _ = highgui::wait_key(1);

    Ok(result_image)
    // Err(Error::new(0, "Failed to mask image"))
}
