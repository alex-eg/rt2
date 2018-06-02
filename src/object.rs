use na::Vector3 as Vec3;
use geometry::Shape;
use material::Material;

use raytracer::Ray;
use light::Light;

#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub shapes: Vec<Shape>,
    pub mat: Material,
}

impl Object {
    pub fn compute_color(&self, ray: &Ray, tnear: f64, nhit: Vec3<f64>,
                         light: &Light) -> Vec3<f64> {
        self.mat.compute_color(ray, tnear, nhit, light)
    }
}

pub fn new_sphere(name: &str, center: Vec3<f64>, radius: f64, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Shape::Sphere { center,
                                    radius, }],
        mat,
    }
}

pub fn new_box(name: &str, vmin: Vec3<f64>, vmax: Vec3<f64>, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Shape::Box { vmin, vmax }],
        mat,
    }
}

pub fn new_triangle(name: &str, a: Vec3<f64>, b: Vec3<f64>, c: Vec3<f64>, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Shape::Triangle { a, b, c }],
        mat
    }
}

pub fn new_square(name: &str, center: Vec3<f64>, size: u16, mat: Material) -> Object {
    let s = size as f64 / 2.;
    let a = Vec3::new(center.x - s, center.y, center.z - s);
    let b = Vec3::new(center.x + s, center.y, center.z - s);
    let c = Vec3::new(center.x + s, center.y, center.z + s);
    let d = Vec3::new(center.x - s, center.y, center.z + s);
    Object {
        name: name.to_string(),
        shapes: vec![Shape::Triangle { a, b, c },
                     Shape::Triangle { a, b: c, c: d }],
        mat,
    }
}


pub struct BoxBuilder {
    boxes: Vec<Shape>
}

impl BoxBuilder {
    pub fn new() -> BoxBuilder {
        BoxBuilder{ boxes: Vec::new() }
    }

    pub fn add(mut self, x: i32, y: i32, z: i32, size: i32) -> BoxBuilder {
        assert!(size > 0);
        let new_box = Shape::Box {
            vmin: Vec3::new(x as f64,
                            y as f64,
                            z as f64),
            vmax: Vec3::new((x + size) as f64,
                            (y + size) as f64,
                            (z + size) as f64) };
        self.boxes.push(new_box);
        self
    }

    pub fn build(self, name: &str, mat: Material) -> Object {
        Object { name: name.to_string(), shapes: self.boxes, mat }
    }
}
