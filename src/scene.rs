use crate::object::Object;
use crate::light::Light;

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
