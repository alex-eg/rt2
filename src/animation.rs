use crate::Vec3;

pub trait SetPosition {
    fn set_position(&mut self, pos: Vec3<f32>);
    fn get_position(&self) -> Vec3<f32>;
}

pub struct Animation {
    path_lengts: Vec<f32>,
    control_points: Vec<Vec3<f32>>,
    current: f32,
    i: usize,
    dir: Vec3<f32>,
    pub dirty: bool,
    factor: f32,
}

impl Animation {
    pub fn new<T: SetPosition>(object: &T, path: &[Vec3<f32>]) -> Animation {
        let mut control_points = Vec::new();
        let pos = object.get_position();
        control_points.push(pos);
        let _ = path.iter().fold(pos, |acc, p| {
            control_points.push(acc + p);
            acc + p
        });
        let mut path_lengts: Vec<f32> = path.iter().map(|p| p.norm()).collect();
        let lastpath = &path.iter().fold(Vec3::new(0., 0., 0.), |acc, p| acc + p);
        path_lengts.push(lastpath.norm());
        let dir = control_points[1] - pos;
        Animation {
            dirty: false,
            control_points,
            current: path_lengts[0],
            path_lengts,
            i: 0,
            dir,
            factor: 1. / 10.,
        }
    }

    pub fn update<T: SetPosition>(&mut self, object: &mut T) {
        self.dirty = true;
        self.current -= self.factor;
        if self.current < 0. {
            self.i += 1;
            if self.i >= self.path_lengts.len() {
                self.i = 0;
            }
            self.current = self.path_lengts[self.i];
            if self.i + 1 == self.path_lengts.len() {
                self.dir = self.control_points[0] - self.control_points[self.i];
            } else {
                self.dir = self.control_points[self.i + 1] - self.control_points[self.i];
            }
        } else {
            let pos = object.get_position() + self.dir.normalize() * self.factor;
            object.set_position(pos);
        }
    }
}
