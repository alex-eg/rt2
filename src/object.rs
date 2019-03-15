use crate::animation::SetPosition;
use crate::geometry::{Shape, Sphere, Cuboid, Triangle};
use crate::material::Material;
use crate::math::{Vec3f, Mat4f, translation, set_translation};
use crate::animation::Animation;

use serde::{Serialize};

#[derive(Serialize)]
pub struct Object {
    pub name: String,
    pub shapes: Vec<Box<dyn Shape>>,
    pub mat: Material,
    pub transform: Mat4f,
    pub animation: Option<Animation>,
}

impl Object {
    pub fn update_animation(&mut self) {
        let mut t: Vec3f = self.get_position();
        match self.animation
        {
            None => {},
            Some(ref mut a) => {
                t += a.update();
            }
        }
        self.set_position(t);
    }

    pub fn set_animation(&mut self, a: Animation) {
        self.animation = Some(a);
    }
}

impl SetPosition for Object {
    fn set_position(&mut self, pos: Vec3f) {
        set_translation(&mut self.transform, pos);
    }

    fn get_position(&self) -> Vec3f {
        translation(&self.transform)
    }
}

pub fn new_sphere(name: &str, center: Vec3f, radius: f32, mat: Material) -> Object {
    let mut t = Mat4f::identity();
    set_translation(&mut t, center);
    Object {
        name: name.to_string(),
        shapes: vec![Box::new(Sphere { radius })],
        mat,
        transform: t,
        animation: None,
    }
}

pub fn new_box(name: &str, vmin: Vec3f, vmax: Vec3f, mat: Material) -> Object {
    let mut t = Mat4f::identity();
    set_translation(&mut t, vmin);
    Object {
        name: name.to_string(),

        shapes: vec![Box::new(Cuboid { extent: vmax })],
        mat,
        transform: t,
        animation: None,
    }
}

pub fn new_triangle(name: &str, a: Vec3f, b: Vec3f, c: Vec3f, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Box::new(Triangle { a, b, c })],
        mat,
        transform: Mat4f::identity(),
        animation: None,
    }
}

pub fn new_square(name: &str, center: Vec3f, size: u16, mat: Material) -> Object {
    let s_2 = f32::from(size) / 2.;
    let a = Vec3f::new(center.x - s_2, center.y, center.z - s_2);
    let b = Vec3f::new(center.x + s_2, center.y, center.z - s_2);
    let c = Vec3f::new(center.x + s_2, center.y, center.z + s_2);
    let d = Vec3f::new(center.x - s_2, center.y, center.z + s_2);
    Object {
        name: name.to_string(),
        shapes: vec![
            Box::new(Triangle { a, b, c }),
            Box::new(Triangle { a, b: c, c: d }),
        ],
        mat,
        transform: Mat4f::identity(),
        animation: None,
    }
}
