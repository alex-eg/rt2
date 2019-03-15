use crate::object::Object;
use crate::light::Light;

use serde::Serialize;

#[derive(Serialize)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn update_objects(&mut self) {
        for obj in &mut self.objects {
            obj.update_animation();
        }
    }
}
