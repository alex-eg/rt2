use na::Vector3 as Vec3;

#[derive(Clone, Copy)]
pub struct Light {
    pub pos: Vec3<f64>,
    pub color: Vec3<f64>,
}
