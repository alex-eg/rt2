use na::Vec3;
use std::f64::INFINITY;
use na::{Norm, Cross};
use camera::Camera;
use light::Light;
use object::Object;

pub struct Ray {
    pub dir: Vec3<f64>,
    pub origin: Vec3<f64>
}

pub trait ComputeColor {
    fn compute_color(&self, &Ray, tnear: f64, &Vec<Box<Object>>) -> Vec3<f64>;
}

pub fn march (cam: &Camera, objects: &Vec<Box<Object>>, lights: &Vec<Box<Light>>)
              -> Vec<Vec3<f64>> {
    let aspect: f64 = cam.width as f64 / cam.height as f64;
    let angle = cam.fov.to_radians().tan();
    let inv_width = 1. / cam.width as f64;
    let inv_height = 1. / cam.height as f64;
    let mut pixels: Vec<Vec3<f64>> =
        Vec::with_capacity((cam.height * cam.width) as usize);
    let right = cam.up.cross(&cam.dir).normalize();
    for y in 0..cam.height {
        for x in 0..cam.width {
            let xx = right * (2. * ((x as f64 + 0.5) * inv_width) - 1.)
                * angle * aspect;
            let yy = cam.up * (1. - 2. * ((y as f64 + 0.5) * inv_height))
                * angle;
            let ray = Ray {
                origin: cam.eye,
                dir: (cam.dir + xx + yy).normalize()
            };
            let color = trace(&ray, objects, lights);
            pixels.push(color);
        }
    }
    pixels
}

fn trace(ray: &Ray, objects: &Vec<Box<Object>>, lights: &Vec<Box<Light>>)
         -> Vec3<f64> {
    let mut tnear = INFINITY;
    let mut object: Option<&Box<Object>> = None;
    for i in 0..objects.len() {
        let (mut t0, t1) = objects[i].shape.intersect(ray);
        if t0 < 0. { t0 = t1 };
        if t0 < tnear {
            tnear = t0;
            object = Some(&objects[i]);
        }
    }
    match object {
        None => { Vec3 {x: 0., y: 0.5, z: 1. } } // blue background
        Some(obj) => {
            obj.compute_color(ray, tnear, objects)
        }
    }
}
