use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Vec3<T: Eq + Copy + Debug + Clone> {
    vec: nalgebra::Vector3<T>,
    x: T,
    y: T,
    z: T,
}
