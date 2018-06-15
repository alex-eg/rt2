use camera::Camera;
use light::Light;
use object::Object;
use geometry::Shape;
use surface::{Division, Surface};

use na::Vector3 as Vec3;
use scoped_threadpool::Pool;
use num_cpus;

use std::f32::INFINITY;

pub struct Ray {
    pub dir: Vec3<f32>,
    pub origin: Vec3<f32>
}

pub fn march (cam: &Camera, objects: &[Object], lights: &[Light])
              -> Vec<u8> {

    let mut pool = Pool::new(num_cpus::get() as u32);
    let surf = Surface::new(cam.width, cam.height);

    pool.scoped(|scope| {
        for chunk in surf.divide(160, 120) {
            scope.execute(move || {
                process_part(cam, objects, lights, &chunk);
            });
        }
    });
    surf.pixels
}

const MAX_DEPTH: u8 = 5;

fn process_part(cam: &Camera, objects: &[Object], lights: &[Light],
                chunk: &Division) {
    let aspect: f32 = cam.width as f32 / cam.height as f32;
    let angle = cam.fov.to_radians().tan();
    let inv_width = 1. / cam.width as f32;
    let inv_height = 1. / cam.height as f32;
    let right = cam.up.cross(&cam.dir).normalize();
    for yi in 0..chunk.h {
        for xi in 0..chunk.w {
            let x = xi + chunk.x0;
            let y = yi + chunk.y0;

            let xx = right * (2. * (x as f32 + 0.5) * inv_width - 1.)
                * angle * aspect;
            let yy = cam.up * (1. - 2. * (y as f32 + 0.5) * inv_height)
                * angle;

            let ray = Ray {
                origin: cam.eye,
                dir: (cam.dir + xx + yy).normalize()
            };

            let default_color = Vec3::new(chunk.x0 as f32 / cam.width as f32, chunk.y0 as f32/ cam.height as f32, 0.7);
            let mut color = trace(&ray, objects, lights, &default_color, 0);
            chunk.set_color(x, y, color);
        }
    }
}

fn hit(ray: &Ray, shape: &Shape) -> (f32, f32) {
    let (t0, t1) = shape.intersect(ray);
    if t0 < 0. { (t1, t0) } else { (t0, t1) }
}

fn trace(ray: &Ray, objects: &[Object], lights: &[Light], default_color: &Vec3<f32>, depth: u8) -> Vec3<f32> {
    if depth > MAX_DEPTH {
        return Vec3::new(0.5, 0.5, 0.5);
    }
    let (mut tnear, mut tfar) = (INFINITY, INFINITY);
    let mut hit_obj: &Object = &objects[0];
    let mut hit_shape: &Shape = &hit_obj.shapes[0];
    for obj in objects {
        for shape in obj.shapes.iter() {
            let (t1, t2) = hit(ray, shape);
            if t1 < tnear {
                tnear = t1;
                tfar = t2;
                hit_shape = &shape;
                hit_obj = &obj;
            }
        }
    }
    let mut color = *default_color;
    if tnear != INFINITY {
        color = Vec3::new(0., 0., 0.,);
        let nhit = hit_shape.get_normal(ray, tnear);
        let phit = ray.origin + ray.dir * tnear;
        for light in lights {
            let mut light_shaded = false;
            'shade: for obj in objects {
                for shape in obj.shapes.iter() {
                    let shadow_ray = Ray {
                        origin: phit + nhit * 0.001,
                        dir: (light.pos - phit).normalize() };
                    let (t1, _) = hit(&shadow_ray, shape);
                    if t1 != INFINITY {
                        light_shaded = true;
                        break 'shade;
                    }
                }
            }
            let mut reflected_color = Vec3::new(0., 0., 0.);
            if hit_obj.mat.reflection > 0.0 {
                let reflection_ray = Ray {
                    origin: phit + nhit * 0.001,
                    dir: ray.dir - 2. * nhit * nhit.dot(&ray.dir)
                };
                reflected_color = trace(&reflection_ray, objects, lights, &Vec3::new(0.0, 0.2, 0.4), depth + 1);
            }

            let mut refracted_color = Vec3::new(0., 0., 0.);
            if hit_obj.mat.refraction > 0.0 {
                // Snell's law
                let n2 = hit_obj.mat.refraction;

                let dot_in = nhit.dot(&ray.dir);
                let factor_in = (n2 * n2 / (dot_in * dot_in) + 1.).sqrt() - 1.;
                let ray_in = Ray {
                    origin: phit + nhit * 0.001,
                    dir: ray.dir + nhit * dot_in * factor_in
                };
                let (_, tfar_in) = hit(&ray_in, hit_shape);
                let nhit_in = hit_shape.get_normal(&ray_in, tfar);

                let dot_out = nhit_in.dot(&ray_in.dir);
                let factor_out = ( -n2 * n2 / (dot_out * dot_out) + 1.).sqrt() - 1.;
                let refraction_ray = Ray {
                    origin: ray_in.origin + ray_in.dir * tfar_in - nhit_in * 0.001,
                    dir: ray_in.dir + nhit_in * dot_out * factor_out
                };
                refracted_color = trace(&refraction_ray, objects, lights, &Vec3::new(0.0, 0.2, 0.4), depth + 1);
            }
            color += hit_obj.mat.compute_color(ray, tnear, nhit, light, reflected_color, refracted_color, light_shaded);
        }
    }
    color
}
