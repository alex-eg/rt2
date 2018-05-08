use na::Vector3 as Vec3;
use std::f64::INFINITY;
use camera::Camera;
use light::Light;
use object::Object;
use std::sync::Arc;

extern crate num_cpus;
extern crate crossbeam_utils;

pub struct Ray {
    pub dir: Vec3<f64>,
    pub origin: Vec3<f64>
}

pub fn march (cam: &Camera, objects: &Vec<Box<Object>>, lights: &Vec<Box<Light>>)
              -> Vec<f64> {
    let pixel_num = (cam.height * cam.width * 3) as usize;
    let mut pixels: Vec<f64> = Vec::with_capacity(pixel_num);
    unsafe {
        pixels.set_len(pixel_num);
    }
    let num_cpus = num_cpus::get();
    let t_cam = Arc::new((*cam).clone());
    let t_obj = Arc::new((*objects).clone());
    let t_lights = Arc::new((*lights).clone());
    let mut chunk_num = 0;
    for chunk in pixels.chunks_mut(pixel_num / num_cpus) {
        let w_cam = t_cam.clone();
        let w_obj = t_obj.clone();
        let w_lights = t_lights.clone();
        let w_num_cpus = num_cpus;
        let w_i = chunk_num;
        crossbeam_utils::scoped::scope(|scope| {
            scope.spawn(move || {
                process_part(w_cam, w_obj, w_lights,
                             w_num_cpus, w_i, chunk);
            });
        });
        chunk_num += 1;
    }
    pixels
}

fn process_part(cam: Arc<Camera>, objects: Arc<Vec<Box<Object>>>, lights: Arc<Vec<Box<Light>>>,
                num_cpus: usize, part_num: usize, pixels: &mut [f64]) {
    let aspect: f64 = cam.width as f64 / cam.height as f64;
    let angle = cam.fov.to_radians().tan();
    let inv_width = 1. / cam.width as f64;
    let inv_height = 1. / cam.height as f64;
    let right = cam.up.cross(&cam.dir).normalize();
    let part_height = cam.height / num_cpus as u32;
    for y in 0..part_height {
        for x in 0..cam.width {
            let xx = right * (2. * (x as f64 + 0.5) * inv_width - 1.)
                * angle * aspect;
            let yy = cam.up * (1. - 2. * (y as f64 + (part_height as f64 * part_num as f64 + 0.5)) * inv_height)
                * angle;
            let ray = Ray {
                origin: cam.eye,
                dir: (cam.dir + xx + yy).normalize()
            };
            let mut color = trace(&ray, objects.clone(), lights.clone());
            if color == Vec3::new(0., 0.5, 1.) {
                color = Vec3::new(0., part_num as f64 / num_cpus as f64, 0.7);
            }
            let index = ((y * cam.width + x) * 3) as usize;
            pixels[index] = color.x;
            pixels[index + 1] = color.y;
            pixels[index + 2] = color.z;
        }
    }
}

fn trace(ray: &Ray, weak_objects: Arc<Vec<Box<Object>>>, weak_lights: Arc<Vec<Box<Light>>>) -> Vec3<f64> {
    let objects = weak_objects;
    let lights = weak_lights;

    let mut tnear = INFINITY;
    for i in 0..objects.len() {
        let (mut t0, t1) = objects[i].shape.intersect(ray);
        if t0 < 0. { t0 = t1 };
        if t0 < tnear {
            tnear = t0;
            let nhit = objects[i].shape.get_normal(ray, tnear);
            return objects[i].compute_color(ray, tnear, nhit, &*lights);
        }
    }
    Vec3::new(0., 0.5, 1.) // blue background
}
