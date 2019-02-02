use crate::animation::SetPosition;
use crate::geometry::{Shape, Sphere, Cuboid, Triangle};
use crate::material::Material;
use crate::math::{Vec3f, Mat4f};

pub struct Object {
    pub name: String,
    pub shapes: Vec<Box<dyn Shape>>,
    pub mat: Material,
    pub transform: Mat4f,
}

impl SetPosition for Object {
    fn set_position(&mut self, pos: Vec3f) {
        self.transform[(0, 3)] = pos.x;
        self.transform[(1, 3)] = pos.y;
        self.transform[(2, 3)] = pos.z;
    }

    fn get_position(&self) -> Vec3f {
        let slice = self.transform.column(3);
        Vec3f::new(slice[0], slice[1], slice[2])
    }
}

pub fn new_sphere(name: &str, center: Vec3f, radius: f32, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Box::new(Sphere { center, radius })],
        mat,
        transform: Mat4f::identity(),
    }
}

pub fn new_box(name: &str, vmin: Vec3f, vmax: Vec3f, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Box::new(Cuboid { vmin, vmax })],
        mat,
        transform: Mat4f::identity(),
    }
}

pub fn new_triangle(name: &str, a: Vec3f, b: Vec3f, c: Vec3f, mat: Material) -> Object {
    Object {
        name: name.to_string(),
        shapes: vec![Box::new(Triangle { a, b, c })],
        mat,
        transform: Mat4f::identity(),
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
    }
}

pub struct CuboidBuilder {
    boxes: Vec<Box<dyn Shape>>,
}

impl CuboidBuilder {
    pub fn new() -> CuboidBuilder {
        CuboidBuilder { boxes: Vec::new() }
    }

    pub fn add(mut self, x: i32, y: i32, z: i32, size: i32) -> CuboidBuilder {
        assert!(size > 0);
        let new_box = Cuboid {
            vmin: Vec3f::new(x as f32, y as f32, z as f32),
            vmax: Vec3f::new((x + size) as f32, (y + size) as f32, (z + size) as f32),
        };
        self.boxes.push(Box::new(new_box));
        self
    }

    pub fn build(self, name: &str, mat: Material) -> Object {
        Object {
            name: name.to_string(),
            shapes: self.boxes,
            mat,
            transform: Mat4f::identity(),
        }
    }
}
