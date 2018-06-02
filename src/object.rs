use na::Vector3 as Vec3;
use geometry::Shape;
use material::Material;

use raytracer::Ray;
use light::Light;

#[derive(Clone)]
pub struct Object {
    pub shapes: Vec<Shape>,
    pub mat: Material,
}

impl Object {
    pub fn compute_color(&self, ray: &Ray, tnear: f64, nhit: Vec3<f64>,
                         lights: &[Light]) -> Vec3<f64> {
        self.mat.compute_color(ray, tnear, nhit, lights)
    }
}

pub fn new_sphere(center: Vec3<f64>, radius: f64, mat: Material) -> Object {
    Object {
        shapes: vec![Shape::Sphere { center,
                                    radius, }],
        mat,
    }
}

pub fn new_box(vmin: Vec3<f64>, vmax: Vec3<f64>, mat: Material) -> Object {
    Object {
        shapes: vec![Shape::Box { vmin, vmax }],
        mat,
    }
}

pub fn new_triangle(a: Vec3<f64>, b: Vec3<f64>, c: Vec3<f64>, mat: Material) -> Object {
    Object {
        shapes: vec![Shape::Triangle { a, b, c }],
        mat
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

    pub fn build(self, mat: Material) -> Object {
        Object { shapes: self.boxes, mat }
    }
}
