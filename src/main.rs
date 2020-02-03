use opencv::{
    core::{Mat, Point},
    highgui,
};

use std::{sync::mpsc, thread};

mod collision;
mod distance;
mod stereo;
use self::collision::CollisionDetector;
use self::distance::UltrasonicSensor;
use self::stereo::StereoView;

const PIN_TRIG: u32 = 14;
const PIN_ECHO: u32 = 15;

fn main() -> opencv::Result<()> {
    let window_title = "enhanced-vision";
    highgui::named_window(window_title, highgui::WINDOW_AUTOSIZE)?;

    let mut stereobm = StereoView::new(0, 5).unwrap();

    let mut origin_depth = 0.0;
    let sonic = UltrasonicSensor::new(PIN_TRIG, PIN_ECHO).unwrap();
    let (tx, rx) = mpsc::channel();
    // handle the distance sensor in a thread because the poll could take multiple milliseconds
    thread::spawn(move || {
        loop {
            let dist = sonic.poll().unwrap();
            // only fails if the receiving end is dropped, in which case we should stop
            if tx.send(dist).is_err() {
                break;
            }
        }
    });

    let view_size = stereobm.get_view_size()?;
    let depth_point = Point::new((view_size[0] / 2.0) as i32, (view_size[1] * 0.2) as i32);
    let mut collide = CollisionDetector::new(view_size[0], view_size[1]);
    loop {
        if let Ok(val) = rx.try_recv() {
            origin_depth = val;
        }

        let mut depth = Mat::default()?;
        stereobm.get_depthmap(&mut depth)?;

        let relative = depth.at_2d(depth_point.x, depth_point.y)?;
        if let Some(hit) = collide.feed_depth_map(&depth, *relative, origin_depth)? {
            println!("collision detected: {:?}", hit)
        }

        let mut visual = Mat::default()?;
        //CV_U8 not defined for some reason...
        depth.convert_to(&mut visual, /*core::CV_U8*/ 0, 1.0, 0.0)?;

        highgui::imshow(window_title, &visual)?;
    }
}
