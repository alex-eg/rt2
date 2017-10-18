use na::Vector3 as Vec3;
use geometry::Shape;
use material::Material;

use raytracer::Ray;
use light::Light;

pub struct Object<'a> {
    pub shape: Shape,
    pub mat: &'a Material,
}

impl<'a> Object<'a> {
    pub fn compute_color(&self, ray: &Ray, tnear: f64, nhit: Vec3<f64>,
                         objects: &Vec<Box<Object>>,
                         lights: &Vec<Box<Light>>) -> Vec3<f64> {
        self.mat.compute_color(ray, tnear, nhit, objects, lights)
    }
}

pub fn new_sphere(center: Vec3<f64>, radius: f64, mat: &Material)
                  -> Object {
    Object {
        shape: Shape::Sphere { center: center,
                               radius: radius, },
        mat: mat,
    }
}

pub fn new_box(vmin: Vec3<f64>, vmax: Vec3<f64>, mat: &Material)
               -> Object {
    Object {
        shape: Shape::Box { vmin: vmin, vmax: vmax },
        mat: mat,
    }
}

pub fn shape_to_obect_vector<'a>(shapes: &Vec<Box<Shape>>, mat: &'a Material)
                             -> Vec<Box<Object<'a>>> {
    let mut obj_vec = Vec::new();
    for s in shapes {
        let obj_box = Box::new(Object { shape: **s,
                                        mat: mat, });
        obj_vec.push(obj_box);
    }
    obj_vec
}
