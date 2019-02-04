type FloatType = f32;

pub use crate::na::Vector3 as Vec3;
pub use crate::na::Vector4 as Vec4;
pub type Vec3f = Vec3<FloatType>;

use crate::na::Matrix4 as Mat4;
pub type Mat4f = Mat4<FloatType>;

pub fn translation(transform: &Mat4f) -> Vec3f {
    let slice = transform.column(3);
    Vec3f::new(slice[0], slice[1], slice[2])
}

pub fn set_translation(transform: &mut Mat4f, translation: Vec3f) {
    transform[(0, 3)] = translation.x;
    transform[(1, 3)] = translation.y;
    transform[(2, 3)] = translation.z;
}
