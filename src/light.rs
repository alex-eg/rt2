use crate::animation::SetPosition;
use crate::math::Vec3;

#[derive(Clone, Copy)]
pub struct Light {
    pub pos: Vec3<f32>,
    pub color: Vec3<f32>,
}

impl SetPosition for Light {
    fn set_position(&mut self, pos: Vec3<f32>) {
        self.pos = pos;
    }

    fn get_position(&self) -> Vec3<f32> {
        self.pos
    }
}
