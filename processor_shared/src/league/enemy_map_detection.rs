use opencv::core::{CV_8UC1, Point3f, Rect};
use opencv::highgui;
use opencv::imgproc::rectangle;
use opencv::{
    core::{Scalar, in_range},
    imgproc::{HOUGH_GRADIENT, hough_circles},
    prelude::*,
};

use crate::{opencv_bullshit_colour_from_rgba, save_as_image};

type Error = opencv::error::Error;

pub struct Detections {
    pub total: u8,
    pub enemies: Vec<Vec<f32>>,
}

impl Detections {
    pub fn from_empty() -> Detections {
        Detections {
            total: 0,
            enemies: Vec::new(),
        }
    }
}

pub fn create_enemy_red_map(image: &Mat) -> Result<Mat, Error> {
    let image_src = image.clone();
    let mut result_image = Mat::new_rows_cols_with_default(
        image.rows(),
        image.cols(),
        CV_8UC1,
        Scalar::new(0., 0., 0., 255.),
    )?;

    // in_range(
    //     &image_src,
    //     &opencv_bullshit_colour_from_rgba(210, 58, 49, 255),
    //     &opencv_bullshit_colour_from_rgba(236, 84, 69, 255),
    //     &mut result_image,
    // )?; //atp masked image will only contain the white pixels of the enemy outlines

    //ignore the turret numbers
    // top turret

    in_range(
        &image_src,
        &opencv_bullshit_colour_from_rgba(210, 58, 49, 255),
        &opencv_bullshit_colour_from_rgba(236, 84, 69, 255),
        &mut result_image,
    )?; //atp masked image will only contain the white pixels of the enemy outlines
    let mut cropped_image = Mat::roi(
        &result_image,
        Rect {
            x: 1498,
            y: 658,
            width: 422,
            height: 422,
        },
    ).unwrap().clone_pointee();
    
    let ignore_rect = Rect {
        x: 130, // top-left x
        y: 41,  // top-left y
        width: 8,
        height: 9,
    };
    rectangle(
        &mut cropped_image,
        ignore_rect,
        Scalar::new(0., 0., 0., 255.), // black out the region
        -1,                            // fill the rectangle
        opencv::imgproc::LINE_AA,
        0,
    )?;
    // mid turret
    let ignore_rect = Rect {
        x: 249, // top-left x
        y: 179, // top-left y
        width: 10,
        height: 12,
    };
    rectangle( //and it also tracks whether the enmy is in topside or botside jungle
        &mut cropped_image,
        ignore_rect,
        Scalar::new(0., 0., 0., 255.), // black out the region
        -1,                            // fill the rectangle
        opencv::imgproc::LINE_AA,
        0,
    )?;
    // bot turret
    let ignore_rect = Rect {
        x: 377, // top-left x
        y: 283, // top-left y
        width: 9,
        height: 11,
    };
    rectangle(
        &mut cropped_image,
        ignore_rect,
        Scalar::new(0., 0., 0., 255.), // black out the region
        -1,                            // fill the rectangle
        opencv::imgproc::LINE_AA,
        0,
    )?;
    // highgui::imshow("Map Visualiser 2", &image_src).unwrap();
    Ok(cropped_image)
}

pub fn detect_enemies_on_redmap(image: &Mat) -> Option<Detections> {
    let mut circles = Mat::default();
    let _ = hough_circles(
        &image,
        &mut circles,
        HOUGH_GRADIENT,
        1.5,
        100.0,
        100.0,
        10.,
        15,
        100,
    );

    if circles.total() == 0 {
        return None;
    }

    let mut result = Vec::new();
    for i in 0..(circles.total()) {
        let circle = circles.at_2d::<Point3f>(0, i as i32).unwrap();

        println!("{:?}", circle);

        result.push(vec![circle.x, circle.y, circle.z]);
    }

    Some(Detections {
        total: result.len() as u8,
        enemies: result,
    })
}

#[cfg(test)]
mod tests {
    use crate::save_as_image;

    use super::*;
    use opencv::imgcodecs::IMREAD_COLOR;
    use opencv::imgcodecs::imread;

    #[test]
    fn test_black_image() {
        let image = imread("..\\images\\TestData\\2 results.png", IMREAD_COLOR).unwrap();

        match create_enemy_red_map(&image) {
            Ok(img) => match detect_enemies_on_redmap(&img) {
                Some(count) => {
                    save_as_image(&img, "test.png").expect("bruh");

                    println!("{}", count.total);
                    if count.total == 2 {
                        return;
                    } else {
                        panic!("Didnt get 2 but got something")
                    }
                }
                None => panic!("Didnt get 2"),
            },
            Err(e) => panic!("{e}"),
        }
    }
}
