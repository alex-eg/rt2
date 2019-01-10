use crate::animation::SetPosition;
use crate::geometry::Shape;
use crate::material::Material;
use crate::Vec3;

#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub shapes: Vec<Shape>,
    pub mat: Material,
}

impl SetPosition for Object {
    // TODO. This is stub implementation.
    // We need functional scene graph to properly
    // handle relative positions of hierarchical objects
    fn set_position(&mut self, pos: Vec3<f32>) {
        match self.shapes[0] {
            Shape::Box {
                ref mut vmin,
                ref mut vmax,
            } => {
                let d = *vmax - *vmin;
                *vmin = pos;
                *vmax = pos + d;
            }
            Shape::Sphere { ref mut center, .. } => *center = pos,
            Shape::Triangle { .. } => (),
        }
    }

    fn get_position(&self) -> Vec3<f32> {
        match self.shapes[0] {
            Shape::Box { vmin, .. } => vmin,
            Shape::Sphere { center, .. } => center,
            Shape::Triangle { .. } => Vec3::new(0., 0., 0.),
        }
    }
}

pub fn new_sphere(name: &str, center: Vec3<f32>, radius: f32, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Shape::Sphere { center, radius }],
        mat,
    }
}

pub fn new_box(name: &str, vmin: Vec3<f32>, vmax: Vec3<f32>, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Shape::Box { vmin, vmax }],
        mat,
    }
}

pub fn new_triangle(name: &str, a: Vec3<f32>, b: Vec3<f32>, c: Vec3<f32>, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Shape::Triangle { a, b, c }],
        mat,
    }
}

pub fn new_square(name: &str, center: Vec3<f32>, size: u16, mat: Material) -> Object {
    let s_2 = f32::from(size) / 2.;
    let a = Vec3::new(center.x - s_2, center.y, center.z - s_2);
    let b = Vec3::new(center.x + s_2, center.y, center.z - s_2);
    let c = Vec3::new(center.x + s_2, center.y, center.z + s_2);
    let d = Vec3::new(center.x - s_2, center.y, center.z + s_2);
    Object {
        name: name.to_string(),
        shapes: vec![
            Shape::Triangle { a, b, c },
            Shape::Triangle { a, b: c, c: d },
        ],
        mat,
    }
}

pub struct BoxBuilder {
    boxes: Vec<Shape>,
}

impl BoxBuilder {
    pub fn new() -> BoxBuilder {
        BoxBuilder { boxes: Vec::new() }
    }

    pub fn add(mut self, x: i32, y: i32, z: i32, size: i32) -> BoxBuilder {
        assert!(size > 0);
        let new_box = Shape::Box {
            vmin: Vec3::new(x as f32, y as f32, z as f32),
            vmax: Vec3::new((x + size) as f32, (y + size) as f32, (z + size) as f32),
        };
        self.boxes.push(new_box);
        self
    }

    pub fn build(self, name: &str, mat: Material) -> Object {
        Object {
            name: name.to_string(),
            shapes: self.boxes,
            mat,
        }
    }
}
