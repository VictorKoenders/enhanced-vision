use opencv::{
    prelude::*,
    videoio,
    core,
    calib3d,
};

pub struct StereoView {
    left: videoio::VideoCapture,
    right: videoio::VideoCapture,
}

impl StereoView {
    pub fn new(left_cam_index: i32, right_cam_index: i32) -> Option<StereoView> {
        let left: videoio::VideoCapture;
        if let Ok(cam) = videoio::VideoCapture::new_with_backend(left_cam_index, videoio::CAP_ANY) {
            if videoio::VideoCapture::is_opened(&cam).unwrap() {
                left = cam;
            }
            else {
                return None; 
            }
        }
        else {
            return None;
        }
        
        let right: videoio::VideoCapture;
        if let Ok(cam) = videoio::VideoCapture::new_with_backend(right_cam_index, videoio::CAP_ANY) {
            if videoio::VideoCapture::is_opened(&cam).unwrap() {
                right = cam;
            }
            else {
                return None;
            }
        }
        else {
            return None;
        }

        Some(StereoView {
            left: left,
            right: right,
        })
    }

    pub fn get_depthmap(&mut self, output: &mut core::Mat) -> opencv::Result<()> {
        use calib3d::StereoMatcher; 
        let mut left = core::Mat::default()?;
        self.left.read(&mut left)?;

        let mut right = core::Mat::default()?;
        self.right.read(&mut right)?;

        let mut stereobm = calib3d::StereoBM::create(0, 21)?;
        //let mut raw = core::Mat::default()?;
        //stereobm.compute(&mut left, &mut right, &mut raw)?;
        stereobm.compute(&mut left, &mut right, output)?;

        Ok(())
    }

    pub fn get_view_size(&self) -> opencv::Result<[f64; 2]> {
        let width = self.left.get(videoio::CAP_PROP_FRAME_WIDTH)?;
        let height = self.left.get(videoio::CAP_PROP_FRAME_HEIGHT)?;
        Ok([width, height])
    }

    pub fn test_cams(&mut self) -> opencv::Result<()> {
        use opencv::highgui;
        
        let left_title = "left";
        let right_title = "right";

        highgui::named_window(left_title, highgui::WINDOW_AUTOSIZE)?;
        highgui::named_window(right_title, highgui::WINDOW_AUTOSIZE)?;

        loop {
            let mut left = core::Mat::default()?;
            let mut right = core::Mat::default()?;
            
            self.left.read(&mut left)?;
            self.right.read(&mut right)?;
            
            if left.size()?.width > 0 {
                highgui::imshow(left_title, &left)?;
            }
            if right.size()?.width > 0 {
                highgui::imshow(right_title, &right)?;
            }
            println!("width: {}, height: {}", right.size()?.width, right.size()?.height);
            if highgui::wait_key(10)? > 0 {
                break;
            }
        }

        Ok(())
    }
}
