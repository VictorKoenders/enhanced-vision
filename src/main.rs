use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    prelude::*,
    types,
    videoio,
};

//use seperate function for convenience with opencv result handling
fn run() -> opencv::Result<()> {
    let window_title = "test";
    highgui::named_window(window_title, 1)?;

    let face_config = core::find_file("haarcascades/haarcascade_frontalface_alt.xml", true, false)?;
    let mut cam = videoio::VideoCapture::new_with_backend(0, videoio::CAP_ANY)?;

    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("could not open camera");
    }
    
    let mut detector = objdetect::CascadeClassifier::new(&face_config)?;
    
    loop {
        let mut frame = Mat::default()?;

        cam.read(&mut frame)?;

        //make grayscale
        let mut gray = Mat::default()?;
        imgproc::cvt_color(
            &frame,
            &mut gray,
            imgproc::COLOR_BGR2GRAY,
            0
        )?;

        //use lower res for faster processing
        // let mut reduced = Mat::default()?;
        // imgproc::resize(
        //     &gray,
        //     &mut reduced,
        //     core::Size {
        //         width: 0,
        //         height: 0
        //     },
        //     0.25f64,
        //     0.25f64,
        //     imgproc::INTER_LINEAR
        // )?;

        let mut faces = types::VectorOfRect::new();
        detector.detect_multi_scale(
            &gray,
            &mut faces,
            1.1,
            2,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size {
                width: 30,
                height: 30
            },
            core::Size {
                width: 0,
                height: 0
            }
        )?;
        println!("found {} faces", faces.len());
        for face in faces {
            //draw rectangle around every face
            imgproc::rectangle(
                &mut frame,
                face,
                core::Scalar::new(0f64, -1f64, -1f64, -1f64),
                1,
                8,
                0
            )?;
        }

        highgui::imshow(window_title, &frame)?;

        if highgui::wait_key(10)? > 0 { //esc key
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
