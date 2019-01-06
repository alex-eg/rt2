use crate::light::Light;
use crate::raytracer::Ray;

use self::na::Vector3 as Vec3;
use nalgebra as na;

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: Vec3<f32>,
    pub diffuse: Vec3<f32>,
    pub specular: Vec3<f32>,
    pub shininess: f32,
    pub reflection: f32,
    pub refraction: f32,
}

pub struct Hit<'a> {
    pub ray: &'a Ray,
    pub tnear: f32,
    pub nhit: Vec3<f32>,
}

fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

fn clamp(f: Vec3<f32>) -> Vec3<f32> {
    let mut h = f;
    h.x = if f.x > 1.0 { 1.0 } else { f.x };
    h.y = if f.y > 1.0 { 1.0 } else { f.y };
    h.z = if f.z > 1.0 { 1.0 } else { f.z };
    h
}

impl Material {
    pub fn compute_color(
        &self,
        hit: &Hit,
        light: &Light,
        reflected_color: Vec3<f32>,
        refracted_color: Vec3<f32>,
        light_shaded: bool,
    ) -> Vec3<f32> {
        let mul = |l: &Vec3<f32>, r: &Vec3<f32>| -> Vec3<f32> {
            Vec3::new(l.x * r.x, l.y * r.y, l.z * r.z)
        };

        let ray = hit.ray;
        let tnear = hit.tnear;
        let nhit = hit.nhit;

        let phit = ray.origin + ray.dir * tnear;
        let ldir = (light.pos - phit).normalize();
        let ndotl = nhit.dot(&ldir);
        let lambert = mul(&light.color, &self.diffuse) * max(ndotl, 0.0);

        let halfv = (-ray.dir + ldir).normalize();
        let ndoth = nhit.dot(&halfv);
        let phong = mul(&light.color, &self.specular) * max(ndoth, 0.0).powf(self.shininess);

        if self.reflection == 0.0 {
            if light_shaded {
                clamp(mul(&self.ambient, &self.diffuse))
            } else {
                clamp(lambert + phong + self.ambient)
            }
        } else if light_shaded {
            clamp(
                mul(&self.ambient, &self.diffuse)
                    + reflected_color * self.reflection
                    + refracted_color * (1.0 - self.reflection),
            )
        } else {
            clamp(
                phong
                    + lambert
                    + reflected_color * self.reflection
                    + refracted_color * (1.0 - self.reflection),
            )
        }
    }
}
