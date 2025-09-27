use opencv::core::{
    CV_8UC1, Point3f, Rect,
};
use opencv::{
    core::{Scalar, in_range},
    imgproc::{
        HOUGH_GRADIENT,
        hough_circles, 
    },
    prelude::*,
};

type Error = opencv::error::Error;

const fn opencv_bullshit_colour_from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Scalar {
    Scalar::new(blue as f64, green as f64, red as f64, alpha as f64)
}

pub struct Detections {
    pub total: u8,
    pub enemies: Vec<Vec<f32>>,
}

pub fn create_enemy_red_map(image: &Mat) -> Result<Mat, Error> {
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

    let cropped_image = Mat::roi(
        &result_image,
        Rect {
            x: 1498,
            y: 658,
            width: 422,
            height: 422,
        },
    )
    .unwrap();
    Ok(cropped_image.clone_pointee())
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
        let image = imread(
            "F:\\Nerd Shit\\Rust\\universal_comms_bot\\images\\TestData\\2 results.png",
            IMREAD_COLOR,
        )
        .unwrap();

        match create_enemy_red_map(&image) {
            Ok(img) => match detect_enemies_on_redmap(&img) {
                Some(count) => {
                    save_as_image(&img, "test.png");

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
