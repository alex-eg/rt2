use na::Vec3;
use std::f64::consts::PI;
use std::f64::INFINITY;
use na::Norm;

use super::{HEIGHT, WIDTH};

struct Ray {
    dir: Vec3<f64>,
    origin: Vec3<f64>
}

struct Sphere {
    radius: f64,
    origin: Vec3<f64>
}

struct Camera {
    up: Vec3<f64>,
    eye: Vec3<f64>,
    look_at: Vec3<f64>,
    fov: f64,
    width: u32,
    height: u32
}

fn march (cam: Camera, spheres: &Vec<Box<Sphere>>) -> Vec<Vec3<u8>> {
    let aspect: f64 = cam.width as f64 / cam.height as f64;
    let angle = (PI * 0.5 * cam.fov / 180.).tan();
    let invWidth = 1. / cam.width as f64;
    let invHeight = 1. / cam.height as f64;
    let mut pixels: Vec<Vec3<u8>> = Vec::with_capacity((cam.height * cam.width) as usize);

    for x in 0..cam.width {
        for y in 0..cam.height {
            let xx: f64 = (2. * ((x as f64 + 0.5) * invWidth) - 1.) * angle * aspect;
            let yy: f64 = (1. - 2. * ((y as f64 + 0.5) * invHeight)) * angle;
            let mut ray: Vec3<f64> = Vec3 { x: xx, y: yy, z: -1. };
            ray.normalize_mut();
            let (r, g, b) = trace(&cam.eye, &ray, spheres);
            pixels[(x * cam.width + y) as usize].x = r;
            pixels[(x * cam.width + y) as usize].y = g;
            pixels[(x * cam.width + y) as usize].z = b;
        }
    }
    pixels
}

fn trace(oriign: &Vec3<f64>, dir: &Vec3<f64>, spheres: &Vec<Box<Sphere>>)
         -> (u8, u8, u8) {
    let mut near: f64 = INFINITY;
    let mut t0: f64;
    let mut t1: f64;
    for sphere in spheres {

    }
    (6, 6, 6)
}
