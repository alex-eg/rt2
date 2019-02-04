use crate::camera::Camera;
use crate::geometry::Shape;
use crate::light::Light;
use crate::object::Object;
use crate::surface::{Division, Surface};
use crate::material::Hit;
use crate::math::{Vec3f, Mat4f};

use num_cpus;
use scoped_threadpool::Pool;

use std::f32::INFINITY;

pub struct Ray {
    pub dir: Vec3f,
    pub origin: Vec3f,
}

struct Params {
    max_depth: u8,
    aspect: f32,
    angle: f32,
    inv_width: f32,
    inv_height: f32,
    right: Vec3f,
}

pub fn march(cam: &Camera, objects: &[Object], lights: &[Light]) -> Vec<u8> {
    let mut pool = Pool::new(num_cpus::get() as u32);
    let surf = Surface::new(cam.width, cam.height);

    let params = Params {
        max_depth: 5,
        aspect: cam.width as f32 / cam.height as f32,
        angle: cam.fov.to_radians().tan(),
        inv_width: 1. / cam.width as f32,
        inv_height: 1. / cam.height as f32,
        right: cam.up.cross(&cam.dir).normalize(),
    };
    pool.scoped(|scope| {
        for chunk in surf.divide(128, 128) {
            scope.execute(|| {
                process_part(cam, objects, lights, chunk, &params);
            });
        }
    });
    surf.pixels
}

fn process_part(cam: &Camera, objects: &[Object], lights: &[Light], chunk: Division, params: &Params) {
    let aspect = params.aspect;
    let angle = params.angle;
    let inv_width = params.inv_width;
    let inv_height = params.inv_height;
    let right = params.right;

    let default_color: Vec3f = Vec3f::new(
        chunk.x0 as f32 / cam.width as f32,
        chunk.y0 as f32 / cam.height as f32,
        0.7,
    );

    for yi in 0..chunk.h {
        for xi in 0..chunk.w {
            let x = xi + chunk.x0;
            let y = yi + chunk.y0;

            let xx = right * (2. * (x as f32 + 0.5) * inv_width - 1.) * angle * aspect;
            let yy = cam.up * (1. - 2. * (y as f32 + 0.5) * inv_height) * angle;

            let ray = Ray {
                origin: cam.eye,
                dir: (cam.dir + xx + yy).normalize(),
            };

            let color = trace(&ray, objects, lights, &default_color, 0, params.max_depth);
            chunk.set_color(x, y, color);
        }
    }
}

fn hit(ray: &Ray, transform: &Mat4f, shape: &Box<Shape>) -> (f32, f32) {
    let (t0, t1) = shape.intersect(transform, ray);
    if t0 < 0. {
        (t1, t0)
    } else {
        (t0, t1)
    }
}

fn trace(
    ray: &Ray,
    objects: &[Object],
    lights: &[Light],
    default_color: &Vec3f,
    depth: u8,
    max_depth: u8,
) -> Vec3f {
    if depth > max_depth {
        return Vec3f::new(0.5, 0.5, 0.5);
    }
    let (mut tnear, mut tfar) = (INFINITY, INFINITY);
    let mut hit_obj = &objects[0];
    let mut hit_shape = &hit_obj.shapes[0];
    for obj in objects {
        for shape in obj.shapes.iter() {
            let (t1, t2) = hit(ray, &obj.transform, shape);
            if t1 < tnear {
                tnear = t1;
                tfar = t2;
                hit_shape = shape;
                hit_obj = &obj;
            }
        }
    }
    let mut color = *default_color;
    if tnear != INFINITY {
        color = Vec3f::new(0., 0., 0.);
        let nhit = hit_shape.get_normal(&hit_obj.transform, ray, tnear);
        let phit = ray.origin + ray.dir * tnear;
        for light in lights {
            let mut light_shaded = false;
            'shade: for obj in objects {
                for shape in obj.shapes.iter() {
                    let shadow_ray = Ray {
                        origin: phit + nhit * 0.001,
                        dir: (light.pos - phit).normalize(),
                    };
                    let (t1, _) = hit(&shadow_ray, &obj.transform, shape);
                    if t1 != INFINITY {
                        light_shaded = true;
                        break 'shade;
                    }
                }
            }
            let reflected_color = if hit_obj.mat.reflection > 0.0 {
                let reflection_ray = Ray {
                    origin: phit + nhit * 0.001,
                    dir: ray.dir - 2. * nhit * nhit.dot(&ray.dir),
                };
                trace(
                    &reflection_ray,
                    objects,
                    lights,
                    &Vec3f::new(0.0, 0.2, 0.4),
                    depth + 1,
                    max_depth
                )
            } else {
                Vec3f::new(0., 0., 0.)
            };

            let refracted_color = if hit_obj.mat.refraction > 0.0 {
                // Snell's law
                let n2 = hit_obj.mat.refraction;

                let dot_in = nhit.dot(&ray.dir);
                let factor_in = (n2 * n2 / (dot_in * dot_in) + 1.).sqrt() - 1.;
                let ray_in = Ray {
                    origin: phit + nhit * 0.001,
                    dir: ray.dir + nhit * dot_in * factor_in,
                };
                let (_, tfar_in) = hit(&ray_in, &hit_obj.transform, &hit_shape);
                let nhit_in = hit_shape.get_normal(&hit_obj.transform, &ray_in, tfar);

                let dot_out = nhit_in.dot(&ray_in.dir);
                let factor_out = (-n2 * n2 / (dot_out * dot_out) + 1.).sqrt() - 1.;
                let refraction_ray = Ray {
                    origin: ray_in.origin + ray_in.dir * tfar_in - nhit_in * 0.001,
                    dir: ray_in.dir + nhit_in * dot_out * factor_out,
                };
                trace(
                    &refraction_ray,
                    objects,
                    lights,
                    &Vec3f::new(0.0, 0.2, 0.4),
                    depth + 1,
                    max_depth,
                )
            } else {
                Vec3f::new(0., 0., 0.)
            };
            let h: Hit = Hit { ray, tnear, nhit };
            color += hit_obj.mat.compute_color(
                &h,
                light,
                reflected_color,
                refracted_color,
                light_shaded,
            );
        }
    }
    color
}
