use opencv::{
    prelude::*,
    core,
    imgproc,
    types::{
        VectorOfPoint,
        VectorOfVectorOfPoint,
    },
};

use std::{
    time::Instant,
};

pub struct CollisionDetector {
    previous_shapes: Vec<ObjectInfo>,
    width: f64,
    height: f64,
}

impl CollisionDetector {
    pub fn new(width: f64, height: f64) -> CollisionDetector {
        CollisionDetector {
            previous_shapes: Vec::new(),
            width: width,
            height: height,
        }
    }

    //origin is the location where the distance sensor measures
    pub fn feed_depth_map(&mut self, depth_map: &core::Mat, origin_value: f64, origin_depth: f64) -> opencv::Result<Option<CollisionInfo>> {
        //find_contours is made to detect white-on-black contours, so do an edge detection first
        let mut visual = Mat::default()?;
        //CV_U8 not defined for some reason... need to convert because canny expects an 8 bit single channel image
        depth_map.convert_to(&mut visual, /*core::CV_U8*/ 0, 1.0, 0.0)?;

        let mut edges = Mat::default()?;
        imgproc::canny(&mut visual, &mut edges, 100.0, 200.0, 3, false)?;

        let mut contours = VectorOfVectorOfPoint::new();
        imgproc::find_contours(&mut edges, &mut contours, imgproc::RETR_TREE, imgproc::CHAIN_APPROX_SIMPLE, core::Point::default())?;

        for contour in contours {
            let mut shape = VectorOfPoint::new();
            imgproc::approx_poly_dp(&contour, &mut shape, 2.6, true)?;
            let m = imgproc::moments(shape, false);
            //calculate shape 'center of mass' from moments
            let position = core::Point::new(m.m10 / m.m00, m.m01 / m.m00);
            let new = ObjectInfo {
                shape: shape,
                position: position,
                depth: (depth_map.at_2d(position.x, position.y)? / origin_value) * origin_depth,
                timestamp: Instant::now(),
            };

            for old in &self.previous_shapes {
                //if shape is 95% similar or better
                if imgproc::match_shapes(&old.shape, &new.shape, imgproc::CONTOURS_MATCH_I1, 0.0)? >= 0.95 {
                    return Ok(self.test_collision(old, &new));
                }
            }
        }

        Ok(None)
    }

    //returns time in seconds until collision, negative if no collision
    fn test_collision(&self, old: &ObjectInfo, new: &ObjectInfo) -> Option<CollisionInfo> {
        let delta = old.timestamp.elapsed().as_secs_f64();
        let delta_x = (new.position.x - old.position.x) as f64 / delta;
        let delta_y = (new.position.y - old.position.y) as f64 / delta;
        let delta_z = (new.depth - old.depth) / delta;
        // if the object is standing still or moving away from the screen there is no way it would collide with us
        if delta_z >= 0.0 {
            return None;
        }

        let t = 0.0;
        let x = old.position.x as f64;
        let y = old.position.y as f64;
        let z = old.depth;

        //timeout of 5 seconds
        while t < 5.0 {
            t += 1e-3;
            x = delta_x * t;
            y = delta_y * t;
            z = delta_z * t;
            
            //if the object ends up outside our vision, assume it doesn't collide
            if x < 0.0 || x > self.width || y < 0.0 || y > self.height {
                break
            }
            
            //apparantely the object will stay inside our vision when it reaches depth 0, i.e. it hits the screen
            if z <= 0.0 {
                return Some(CollisionInfo {
                    time_left: t,
                    impact_point: core::Point::new(x as i32, y as i32),
                });
            }
        }

        //timeout elapsed, so no collision
        None
    }
}

struct ObjectInfo {
    shape: VectorOfPoint,
    position: core::Point,
    depth: f64,
    timestamp: Instant,
}

#[derive(Debug)]
pub struct CollisionInfo {
    pub time_left: f64,
    pub impact_point: core::Point,
}
