use crate::math::Vec3f;
use crate::na::zero;

pub trait SetPosition {
    fn set_position(&mut self, pos: Vec3f);
    fn get_position(&self) -> Vec3f;
}

pub struct Animation {
    path_lengts: Vec<f32>,
    control_points: Vec<Vec3f>,
    current: f32,
    i: usize,
    dir: Vec3f,
    pub dirty: bool,
    factor: f32,
}

impl Animation {
    pub fn new<T: SetPosition>(object: &T, path: &[Vec3f]) -> Animation {
        let mut control_points = Vec::new();
        let pos = object.get_position();
        control_points.push(pos);
        let _ = path.iter().fold(pos, |acc, p| {
            control_points.push(acc + p);
            acc + p
        });
        let mut path_lengts: Vec<f32> = path.iter().map(|p| p.norm()).collect();
        let lastpath = &path.iter().fold(Vec3f::new(0., 0., 0.), |acc, p| acc + p);
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

    pub fn update(&mut self) -> Vec3f {
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
                zero()
            } else {
                self.dir = self.control_points[self.i + 1] - self.control_points[self.i];
                zero()
            }
        } else {
            self.dir.normalize() * self.factor
        }
    }
}
