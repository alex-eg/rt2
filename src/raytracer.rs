use camera::Camera;
use light::Light;
use object::Object;
use geometry::Shape;
use surface::{Division, Surface};

use na::Vector3 as Vec3;
use scoped_threadpool::Pool;
use num_cpus;

use std::f64::INFINITY;

pub struct Ray {
    pub dir: Vec3<f64>,
    pub origin: Vec3<f64>
}

pub fn march (cam: &Camera, objects: &[Object], lights: &[Light])
              -> Vec<u8> {

    let mut pool = Pool::new(num_cpus::get() as u32);
    let surf = Surface::new(cam.width, cam.height);

    pool.scoped(|scope| {
        for chunk in surf.divide(32, 32) {
            scope.execute(move || {
                process_part(cam, objects, lights, &chunk);
            });
        }
    });
    surf.pixels
}

fn process_part(cam: &Camera, objects: &[Object], lights: &[Light],
                chunk: &Division) {
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

fn hit(ray: &Ray, shape: &Shape) -> f64 {
    let (t0, t1) = shape.intersect(ray);
    if t0 < 0. { t1 } else { t0 }
}

fn trace(ray: &Ray, objects: &[Object], lights: &[Light]) -> Vec3<f64> {
    let mut tnear = INFINITY;
    let mut hit_obj: &Object = &objects[0];
    let mut hit_shape: &Shape = &hit_obj.shapes[0];
    for obj in objects {
        for shape in obj.shapes.iter() {
            let t = hit(ray, shape);
            if t < tnear {
                tnear = t;
                hit_shape = &shape;
                hit_obj = &obj;
            }
        }
    }
    let mut color = Vec3::new(0., 0., 0.);
    if tnear != INFINITY {
        color = Vec3::new(0.001, 0., 0.);
        let nhit = hit_shape.get_normal(ray, tnear);
        let phit = ray.origin + ray.dir * tnear;
        'light: for light in lights {
            for obj in objects {
                for shape in obj.shapes.iter() {
                    let shadow_ray = Ray {
                        origin: phit + nhit * 0.001,
                        dir: (light.pos - phit).normalize() };
                    if hit(&shadow_ray, shape) != INFINITY {
                        continue 'light;
                    }
                }
            }
            color += hit_obj.compute_color(ray, tnear, nhit, light);
        }
    }
    color
}
