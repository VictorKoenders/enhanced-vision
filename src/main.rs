use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    prelude::*,
    types,
    videoio,
};

mod distance;
use self::distance::UltrasonicSensor;

const PIN_TRIG: u32 = 14;
const PIN_ECHO: u32 = 15;

//use seperate function for convenience with opencv result handling
fn run() -> opencv::Result<()> {
    let left_title = "left";
    let right_title = "right";

    highgui::named_window(left_title, 1)?;
    highgui::named_window(right_title, 1)?;

    let mut left = videoio::VideoCapture::new_with_backend(0, videoio::CAP_ANY)?;
    let mut right = videoio::VideoCapture::new_with_backend(4, videoio::CAP_ANY)?;

    if !(videoio::VideoCapture::is_opened(&left)? && videoio::VideoCapture::is_opened(&right)?) {
        panic!("could not open cameras");
    }
    
    loop {
        let mut left_view = Mat::default()?;
        left.read(&mut left_view)?;

        let mut right_view = Mat::default()?;
        right.read(&mut right_view)?;

        highgui::imshow(left_title, &left_view)?;
        highgui::imshow(right_title, &right_view)?;

        if highgui::wait_key(10)? > 0 { //esc key
            break;
        }
    }
    Ok(())
}

fn main() {
    let dist = UltrasonicSensor::new(PIN_TRIG, PIN_ECHO).unwrap();
    println!("test distance: {:.0}", dist.poll().unwrap());
    //run().unwrap()
}
