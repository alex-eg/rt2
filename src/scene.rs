use crate::object::Object;
use crate::light::Light;
use crate::camera::Camera;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub cam: Camera,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn update_objects(&mut self) {
        for obj in &mut self.objects {
            obj.update_animation();
        }
    }

    pub fn any_animation_dirty(&self) -> bool {
        for o in &self.objects {
            match &o.animation {
                Some(a) => if a.dirty == true { return true },
                None => {}
            }
        }
        false
    }
}
