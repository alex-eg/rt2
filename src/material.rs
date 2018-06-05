use na::Vector3 as Vec3;
use raytracer::Ray;
use light::Light;

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: Vec3<f64>,
    pub diffuse: Vec3<f64>,
    pub specular: Vec3<f64>,
    pub shininess: f64,
    pub reflection: f64,
    pub refraction: f64,
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
    pub fn compute_color(&self, ray: &Ray, tnear: f64, nhit: Vec3<f64>, light: &Light,
                         reflected_color: Vec3<f64>, refracted_color: Vec3<f64>,
                         light_shaded: bool)
                         -> Vec3<f64> {
        let mul = |l: &Vec3<f64>, r: &Vec3<f64>| -> Vec3<f64> {
            Vec3::new(l.x * r.x, l.y * r.y, l.z * r.z)
        };

        let phit = ray.origin + ray.dir * tnear;
        let ldir = (light.pos - phit).normalize();
        let ndotl = nhit.dot(&ldir);
        let lambert = mul(&light.color, &self.diffuse) * max(ndotl, 0.0);

        let halfv = (-ray.dir + ldir).normalize();
        let ndoth = nhit.dot(&halfv);
        let phong = mul(&light.color, &self.specular)
            * max(ndoth, 0.0).powf(self.shininess);

        if self.reflection == 0.0 {
            if light_shaded {
                clamp(mul(&self.ambient, &self.diffuse))
            } else {
                clamp(lambert + phong + self.ambient)
            }
        } else {
            if light_shaded {
                clamp(mul(&self.ambient, &self.diffuse) + reflected_color * self.reflection
                  + refracted_color * (1.0 - self.reflection))
            } else {
                clamp(phong + lambert + reflected_color * self.reflection
                  + refracted_color * (1.0 - self.reflection))
            }
        }
    }
}
