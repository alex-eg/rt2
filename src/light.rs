use crate::animation::SetPosition;
use crate::math::Vec3f;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Light {
    pub pos: Vec3f,
    pub color: Vec3f,
}

impl SetPosition for Light {
    fn set_position(&mut self, pos: Vec3f) {
        self.pos = pos;
    }

    fn get_position(&self) -> Vec3f {
        self.pos
    }
}
