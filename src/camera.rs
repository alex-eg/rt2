use na::{Vector3 as Vec3, Rotation3 as Rot3, Unit};

use num::traits::Zero;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// Camera eye position
    pub eye: Vec3<f64>,
    /// Camera view direction vector
    pub dir: Vec3<f64>,
    pub up: Vec3<f64>,
    pub fov: f64,
    pub width: u32,
    pub height: u32,
}

pub struct CamBuilder {
    /// Camera eye position
    eye: Vec3<f64>,
    /// Point to which camera looks at
    center: Vec3<f64>,
    up: Vec3<f64>,
    /// View direction, i.e., center - eye vector
    fov: f64,
    width: u32,
    height: u32,
}

impl CamBuilder {
    pub fn new() -> CamBuilder {
        CamBuilder {
            eye: Vec3::zero(),
            center: Vec3::zero(),
            up: Vec3::zero(),

            fov: 0.0,
            width: 0,
            height: 0,
        }
    }

    pub fn eye(&mut self, eye: Vec3<f64>) -> &mut CamBuilder {
        self.eye = eye;
        self
    }

    pub fn center(&mut self, center: Vec3<f64>) -> &mut CamBuilder {
        self.center = center;
        self
    }

    pub fn up(&mut self, up: Vec3<f64>) -> &mut CamBuilder {
        self.up = up;
        self
    }

    pub fn fov(&mut self, fov: f64) -> &mut CamBuilder {
        self.fov = fov;
        self
    }

    pub fn width(&mut self, width: u32) -> &mut CamBuilder {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: u32) -> &mut CamBuilder {
        self.height = height;
        self
    }

    pub fn build(&self) -> Camera {
        let dir = self.center - self.eye;
        Camera {
            eye: self.eye,
            dir: dir,
            up: self.up,
            fov: self.fov,
            width: self.width,
            height: self.height
        }
    }
}

impl Camera {
    pub fn roll(&mut self, angle: f64) {
        let rot = Rot3::from_axis_angle(&Unit::new_normalize(self.dir), angle.to_radians());
        self.up = (rot * self.up).normalize();
    }

    pub fn yaw(&mut self, angle: f64) {
        let rot = Rot3::from_axis_angle(&Unit::new_normalize(self.up), angle.to_radians());
        self.dir = rot * self.dir;
    }

    pub fn pitch(&mut self, angle: f64) {
        let side = self.up.cross(&self.dir);
        let rot = Rot3::from_axis_angle(&Unit::new_normalize(side), angle.to_radians());
        self.dir = rot * self.dir;
        self.up = self.dir.cross(&side).normalize();
    }

    pub fn mov_fwd(&mut self, dist: f64) {
        let dir = self.dir;
        self.eye = self.eye + dir * dist;
    }

    pub fn mov_side(&mut self, dist: f64) {
        let dir = self.dir;
        let side = self.up.cross(&dir);
        self.eye = self.eye + side * dist;
    }
}
