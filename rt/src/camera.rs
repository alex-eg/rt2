use na::{Vec3, Rot3};
use na::{Norm, Dot, Cross};

use num_traits::identities::Zero;

#[derive(Debug)]
pub struct Camera {
    /// Camera eye position
    pub eye: Vec3<f64>,
    /// View direction, i.e., center-eye vector
    pub dir: Vec3<f64>,
    /// Point to which camera looks at
    pub center: Vec3<f64>,
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
    dir: Vec3<f64>,
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
            dir: Vec3::zero(),
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
        let dir = (self.center - self.eye).normalize();
        println!("Dir is {}", dir);
        Camera {
            eye: self.eye,
            dir: dir,
            center: self.center,
            up: self.up,
            fov: self.fov,
            width: self.width,
            height: self.height
        }
    }
}

impl Camera {
    pub fn dir(&self) -> Vec3<f64> {
        (self.center - self.eye).normalize()
    }

    pub fn roll(&mut self, angle: f64) {
        let rot = Rot3::new(self.dir() * angle.to_radians());
        self.up = (self.up * rot).normalize();
    }

    pub fn yaw(&mut self, angle: f64) {
        let rot = Rot3::new(self.up * angle.to_radians());
        self.center = self.eye + self.dir() * rot;
    }

    pub fn pitch(&mut self, angle: f64) {
        let dir = self.dir();
        let side = self.up.cross(&dir);
        let rot = Rot3::new(side * angle.to_radians());
        let new_view = dir * rot;
        self.center = self.eye + new_view;
        self.up = new_view.cross(&side).normalize();
    }
}
