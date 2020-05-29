use crate::na::{clamp, Rotation3 as Rot3, Unit};
use crate::math::Vec3f;

use num::traits::Zero;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Rotate {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Camera {
    /// Camera eye position
    pub eye: Vec3f,

    /// Camera view direction vector
    pub dir: Vec3f,

    pub up: Vec3f,
    pub fov: f32,
    pub width: u32,
    pub height: u32,

    angles: Rotate,
}

pub struct CamBuilder {
    /// Camera eye position
    eye: Vec3f,

    /// Point to which camera looks at
    center: Vec3f,

    up: Vec3f,
    fov: f32,
    width: u32,
    height: u32,
}

impl CamBuilder {
    pub fn new() -> CamBuilder {
        CamBuilder {
            eye: Vec3f::zero(),
            center: Vec3f::zero(),
            up: Vec3f::zero(),

            fov: 0.0,
            width: 0,
            height: 0,
        }
    }

    pub fn eye(&mut self, eye: Vec3f) -> &mut CamBuilder {
        self.eye = eye;
        self
    }

    pub fn center(&mut self, center: Vec3f) -> &mut CamBuilder {
        self.center = center;
        self
    }

    pub fn up(&mut self, up: Vec3f) -> &mut CamBuilder {
        self.up = up;
        self
    }

    pub fn fov(&mut self, fov: f32) -> &mut CamBuilder {
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
        let y = Vec3f::new(0.0, 0.0, 1.0).dot(&dir).acos().to_degrees();
        Camera {
            eye: self.eye,
            dir,
            up: self.up,
            fov: self.fov,
            width: self.width,
            height: self.height,
            angles: Rotate { x: 0.0, y },
        }
    }
}

impl Camera {
    pub fn yaw(&mut self, angle: f32) {
        self.angles.y -= angle;
        if self.angles.y < 0.0 {
            self.angles.y += 360.0;
        }
        if self.angles.y > 360.0 {
            self.angles.y -= 360.0;
        }
        self.update();
    }

    pub fn pitch(&mut self, angle: f32) {
        self.angles.x -= angle;
        self.angles.x = clamp(self.angles.x, -89.0, 89.0);
        self.update();
    }

    pub fn update(&mut self) {
        let rot_x = Rot3::from_axis_angle(
            &Unit::new_normalize(Vec3f::new(1.0, 0.0, 0.0)),
            self.angles.x.to_radians(),
        );
        let rot_y = Rot3::from_axis_angle(
            &Unit::new_normalize(Vec3f::new(0.0, 1.0, 0.0)),
            self.angles.y.to_radians(),
        );
        self.dir = rot_y * rot_x * Vec3f::new(0.0, 0.0, 1.0);
        self.up = rot_y * rot_x * Vec3f::new(0.0, -1.0, 0.0);
    }

    pub fn mov_fwd(&mut self, dist: f32) {
        let dir = self.dir;
        self.eye += dir * dist;
    }

    pub fn mov_side(&mut self, dist: f32) {
        let dir = self.dir;
        let mut side = self.up.cross(&dir);
        side.y = 0.0;
        self.eye += side * dist;
    }
}
