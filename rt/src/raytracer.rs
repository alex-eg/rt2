use na::Vec3;
use std::f64::INFINITY;
use na::{Norm, Dot, Cross};
use camera::Camera;

struct Ray {
    dir: Vec3<f64>,
    origin: Vec3<f64>
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec3<f64>,
}

impl Sphere {
    fn intersect(&self, ray: &Ray, t0: &mut f64, t1: &mut f64) -> bool  {
        let l = self.center - ray.origin;
        let tca = l.dot(&ray.dir);
        if tca < 0. { return false; }
        let d2 = l.dot(&l) - tca * tca;
        let r2 = self.radius * self.radius;
        if d2 > r2 { return false; }
        let thc: f64 = (r2 - d2).sqrt();
        *t0 = tca - thc;
        *t1 = tca + thc;
        true
    }
}

pub fn march (cam: &Camera, spheres: &Vec<&Sphere>) -> Vec<Vec3<f64>> {
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
            let yy = cam.up * (1. - 2. * ((y as f64 + 0.5) * inv_height)) * angle;
            let ray = Ray {
                origin: cam.eye,
                dir: (cam.dir + xx + yy).normalize()
            };
            let color = trace(&ray, spheres);
            pixels.push(color);
        }
    }
    pixels
}

fn trace(ray: &Ray, spheres: &Vec<&Sphere>)
         -> Vec3<f64> {
    let mut tnear = INFINITY;
    let mut sphere: Option<&Sphere> = None;
    for i in 0..spheres.len() {
        let mut t0 = INFINITY;
        let mut t1 = INFINITY;
        let _ = spheres[i].intersect(ray, &mut t0, &mut t1);
        if t0 < 0. { t0 = t1 };
        if t0 < tnear {
            tnear = t0;
            sphere = Some(spheres[i]);
        }
    }
    match sphere {
        None => { Vec3 {x: 0., y: 0.5, z: 1. } } // blue background
        Some(sphere) => {
            let phit = ray.origin + ray.dir * tnear;
            let mut nhit = (phit - sphere.center).normalize();
            let bias = 1e-4;
            if ray.dir.dot(&nhit) > 0. {
                nhit = -nhit;
            }

            let mut transmission = 1.0;
            let light_dir = (Vec3 { x: 0., y: 0., z: 10. } - phit)
                .normalize();
            let light_color = Vec3 { x: 1., y: 1., z: 1. };

            'shadow: for i in 0..spheres.len() {
                let (mut t0, mut t1): (f64, f64) = (0., 0.);
                if spheres[i].intersect(&Ray { origin: phit + nhit * bias,
                                               dir: light_dir },
                                        &mut t0, &mut t1) {
                    transmission = 0.0;
                    break 'shadow;
                }
            }
            let color: Vec3<f64> = Vec3 { x: 1.0, y: 0., z: 0. }
                * transmission
                * if nhit.dot(&light_dir) > 0. { nhit.dot(&light_dir) }
                  else { 0. }
                * light_color;
            color
        }
    }
}
