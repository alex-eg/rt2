use na::Vector3 as Vec3;
use geometry::Shape;
use material::Material;

use raytracer::Ray;
use light::Light;

#[derive(Clone, Copy)]
pub struct Object {
    pub shape: Shape,
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
        shape: Shape::Sphere { center: center,
                               radius: radius, },
        mat: mat,
    }
}

pub fn new_box(vmin: Vec3<f64>, vmax: Vec3<f64>, mat: Material) -> Object {
    Object {
        shape: Shape::Box { vmin: vmin, vmax: vmax },
        mat: mat,
    }
}

pub fn new_triangle(a: Vec3<f64>, b: Vec3<f64>, c: Vec3<f64>, mat: Material) -> Object {
    Object {
        shape: Shape::Triangle { a, b, c },
        mat: mat
    }
}

pub fn shape_to_obect_vector(shapes: &[Shape], mat: Material) -> Vec<Object> {
    let mut obj_vec = Vec::new();
    for shape in shapes {
        obj_vec.push(Object { shape: *shape, mat });
    }
    obj_vec
}
