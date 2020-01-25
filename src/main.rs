use opencv::{
    prelude::*,
    highgui,
    core::Mat,
    core,
};

mod distance;
mod stereo;
mod collision;
use self::distance::UltrasonicSensor;

const PIN_TRIG: u32 = 14;
const PIN_ECHO: u32 = 15;

fn main() {
    run().unwrap();
}

fn run() -> opencv::Result<()> {
    let mut stereobm = stereo::StereoView::new(0, 5).unwrap();
    
    loop {
        let mut depth = Mat::default()?;
        stereobm.get_depthmap(&mut depth)?;
    }

    Ok(())
}
