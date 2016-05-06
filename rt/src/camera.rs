use na::Vec3;
use std::f64::consts::PI;
use std::f64::INFINITY;
use na::{Norm, Dot};

use num_traits::identities::Zero;

#[derive(Debug)]
pub struct Camera {
    pub name: String,
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
    name: String,
    /// Camera eye position
    eye: Vec3<f64>,
    /// Point to which camera looks at
    center: Vec3<f64>,
    up: Vec3<f64>,
    /// View direction, i.e., center-eye vector
    dir: Vec3<f64>,
    fov: f64,
    width: u32,
    height: u32,
}

impl CamBuilder {
    pub fn new(name: &str) -> CamBuilder {
        CamBuilder {
            name: name.to_string(),
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

    pub fn dir(&mut self, dir: Vec3<f64>) -> &mut CamBuilder {
        self.dir = dir;
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
        let center =(self.eye + self.dir).normalize();
        Camera {
            name: self.name.clone(),
            eye: self.eye,
            dir: self.dir,
            center: center,
            up: self.up,
            fov: self.fov,
            width: self.width,
            height: self.height
        }
    }
}

impl Camera {
    pub fn dir(&self) -> Vec3<f64> {
        self.center - self.eye
    }
}
