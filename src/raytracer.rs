use camera::Camera;
use light::Light;
use object::Object;
use surface::{Division, Surface};

use na::Vector3 as Vec3;
use scoped_threadpool::Pool;
use num_cpus;

use std::f64::INFINITY;

pub struct Ray {
    pub dir: Vec3<f64>,
    pub origin: Vec3<f64>
}

pub fn march (cam: &Camera, objects: &Vec<Box<Object>>, lights: &Vec<Box<Light>>)
              -> Vec<u8> {

    let mut pool = Pool::new(num_cpus::get() as u32);
    let surf = Surface::new(cam.width, cam.height);

    pool.scoped(|scope| {
        for chunk in surf.divide(32, 32) {
            scope.execute(move || {
                process_part(cam, objects, lights, chunk);
            });
        }
    });
    surf.pixels
}

fn process_part(cam: &Camera, objects: &Vec<Box<Object>>, lights: &Vec<Box<Light>>,
                chunk: Division) {
    let aspect: f64 = cam.width as f64 / cam.height as f64;
    let angle = cam.fov.to_radians().tan();
    let inv_width = 1. / cam.width as f64;
    let inv_height = 1. / cam.height as f64;
    let right = cam.up.cross(&cam.dir).normalize();
    for yi in 0..chunk.h {
        for xi in 0..chunk.w {
            let x = xi + chunk.x0;
            let y = yi + chunk.y0;

            let xx = right * (2. * (x as f64 + 0.5) * inv_width - 1.)
                * angle * aspect;
            let yy = cam.up * (1. - 2. * (y as f64 + 0.5) * inv_height)
                * angle;

            let ray = Ray {
                origin: cam.eye,
                dir: (cam.dir + xx + yy).normalize()
            };

            let mut color = trace(&ray, objects, lights);
            if color == Vec3::new(0., 0., 0.) {
                color = Vec3::new(chunk.x0 as f64 / cam.width as f64, chunk.y0 as f64/ cam.height as f64, 0.7);
            }
            chunk.set_color(x, y, color);
        }
    }
}

fn trace(ray: &Ray, objects: &Vec<Box<Object>>, lights: &Vec<Box<Light>>) -> Vec3<f64> {
    let mut tnear = INFINITY;
    for i in 0..objects.len() {
        let (mut t0, t1) = objects[i].shape.intersect(ray);
        if t0 < 0. { t0 = t1 };
        if t0 < tnear {
            tnear = t0;
            let nhit = objects[i].shape.get_normal(ray, tnear);
            return objects[i].compute_color(ray, tnear, nhit, lights);
        }
    }
    Vec3::new(0., 0., 0.)
}
