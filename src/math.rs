use serde::ser::{Serialize, SerializeStruct};
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Vec3<T: Eq + Copy + Debug + Clone> {
    vec: nalgebra::Vector3<T>,
    x: T,
    y: T,
    z: T,
}

impl<T> Serialize for Vec3<T> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = s.serialize_struct("Vec3", 3)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.end()
    }
}
