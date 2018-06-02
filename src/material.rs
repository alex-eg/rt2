use na::Vector3 as Vec3;
use num::traits::Zero;
use raytracer::Ray;
use light::Light;

#[derive(Clone, Copy)]
pub enum Material {
    Plain {
        color: Vec3<f64>,
    },

    Lambert {
        ambient: Vec3<f64>,
        diffuse: Vec3<f64>,
        specular: Vec3<f64>,
        emission: Vec3<f64>,
        shininess: f64,
    }
}

fn max(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn clamp(f: Vec3<f64>) -> Vec3<f64> {
    let mut h = f;
    h.x = if f.x  > 1.0 { 1.0 } else { f.x };
    h.y = if f.y  > 1.0 { 1.0 } else { f.y };
    h.z = if f.z  > 1.0 { 1.0 } else { f.z };
    h
}

impl Material {
    pub fn compute_color(&self, ray: &Ray, tnear: f64, nhit: Vec3<f64>, light: &Light)
                         -> Vec3<f64> {
        let mul = |l: &Vec3<f64>, r: &Vec3<f64>| -> Vec3<f64> {
            Vec3::new(l.x * r.x, l.y * r.y, l.z * r.z)
        };
        match *self {
            Material::Plain { color } => { color }
            Material::Lambert { ambient, diffuse, specular, emission,
                                shininess } => {
                let mut color: Vec3<f64> = Vec3::zero();
                let phit = ray.origin + ray.dir * tnear;
                let ldir = (light.pos - phit).normalize();
                let ndotl = nhit.dot(&ldir);
                let lambert = mul(&light.color, &diffuse) * max(ndotl, 0.0);

                let halfv = (-ray.dir + ldir).normalize();
                let ndoth = nhit.dot(&halfv);
                let phong = mul(&light.color, &specular)
                    * max(ndoth, 0.0).powf(shininess);

                color = color + lambert + phong;
                clamp(color + ambient + emission)
            }
        }
    }
}
