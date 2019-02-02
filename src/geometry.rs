use crate::raytracer::Ray;
use crate::animation::SetPosition;

use crate::math::Vec3f;
use crate::math::Vec3;

use num::traits::Zero;
use std::f32::INFINITY;

pub trait Shape: Send + Sync {
    fn get_normal(&self, ray: &Ray, tnear: f32) -> Vec3f;
    fn intersect(&self, ray: &Ray) -> (f32, f32);
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub radius: f32,
    pub center: Vec3f,
}

#[derive(Clone, Copy)]
pub struct Cuboid {
    pub vmin: Vec3f,
    pub vmax: Vec3f,
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: Vec3f,
    pub b: Vec3f,
    pub c: Vec3f,
}

impl Shape for Cuboid {
    fn get_normal(&self, ray: &Ray, tnear: f32) -> Vec3f {
        let vmin = self.vmin;
        let vmax = self.vmax;
        let phit = ray.origin + ray.dir * tnear;

        let phit_min = phit - vmin;
        let phit_max = phit - vmax;
        let mut nhit = Vec3::zero();
        let eps = 1e-6;
        if phit_min.x.abs() < eps {
            nhit.x = -1.
        }
        if phit_min.y.abs() < eps {
            nhit.y = -1.
        }
        if phit_min.z.abs() < eps {
            nhit.z = -1.
        }
        if phit_max.x.abs() < eps {
            nhit.x = 1.
        }
        if phit_max.y.abs() < eps {
            nhit.y = 1.
        }
        if phit_max.z.abs() < eps {
            nhit.z = 1.
        }
        nhit
    }

    fn intersect(&self, ray: &Ray) -> (f32, f32) {
        let (mut t0, t1) = (INFINITY, INFINITY);
        let o = ray.origin;
        let mut d = ray.dir;

        d.x = 1. / d.x;
        d.y = 1. / d.y;
        d.z = 1. / d.z;

        let mut sign: Vec3<bool> = Vec3::new(true, true, true);
        sign.x = d.x > 0.;
        sign.y = d.y > 0.;
        sign.z = d.z > 0.;

        let b0 = self.vmin;
        let b1 = self.vmax;

        let mut tmin = (if sign.x { b0.x } else { b1.x } - o.x) * d.x;
        let mut tmax = (if sign.x { b1.x } else { b0.x } - o.x) * d.x;

        let tymin = (if sign.y { b0.y } else { b1.y } - o.y) * d.y;
        let tymax = (if sign.y { b1.y } else { b0.y } - o.y) * d.y;

        if tmin > tymax || tymin > tmax {
            return (t0, t1);
        };
        if tymin > tmin {
            tmin = tymin
        };
        if tymax < tmax {
            tmax = tymax
        };

        let tzmin = (if sign.z { b0.z } else { b1.z } - o.z) * d.z;
        let tzmax = (if sign.z { b1.z } else { b0.z } - o.z) * d.z;

        if tmin > tzmax || tzmin > tmax {
            return (t0, t1);
        };
        if tzmin > tmin {
            tmin = tzmin
        };
        if tzmax < tmax {
            tmax = tzmax
        };

        if tmin < 0. {
            if tmax < 0. {
                return (t0, t1);
            };
            t0 = tmax;
        } else {
            t0 = tmin;
        }
        (t0, t1)
    }
}

impl SetPosition for Cuboid {
    fn set_position(&mut self, pos: Vec3f)
    {
        let d = self.vmax - self.vmin;
        self.vmin = pos;
        self.vmax = pos + d;
    }

    fn get_position(&self) -> Vec3f
    {
        self.vmin
    }
}

impl Shape for Sphere {
    fn get_normal(&self, ray: &Ray, tnear: f32) -> Vec3f {
        let center = self.center;
        let phit = ray.origin + ray.dir * tnear;
        let mut nhit = (phit - center).normalize();
        if ray.dir.dot(&nhit) > 0. {
            nhit = -nhit;
        }
        nhit
    }

    fn intersect(&self, ray: &Ray) -> (f32, f32) {
        let (mut t0, mut t1) = (INFINITY, INFINITY);
        let l = self.center - ray.origin;
        let tca = l.dot(&ray.dir);
        if tca < 0. {
            return (t0, t1);
        }
        let d2 = l.dot(&l) - tca * tca;
        let r2 = self.radius * self.radius;
        if d2 > r2 {
            return (t0, t1);
        }
        let thc: f32 = (r2 - d2).sqrt();
        t0 = tca - thc;
        t1 = tca + thc;
        (t0, t1)
    }
}

impl SetPosition for Sphere {
    fn set_position(&mut self, pos: Vec3f) {
        self.center = pos;
    }

    fn get_position(&self) -> Vec3f {
        self.center
    }
}

impl Shape for Triangle {
    fn get_normal(&self, ray: &Ray, tnear: f32) -> Vec3f {
        let t = ray.origin + ray.dir * tnear;
        let edge_0 = t - self.a;
        let edge_1 = t - self.b;
        let mut nhit = edge_1.cross(&edge_0).normalize();
        if ray.dir.dot(&nhit) > 0. {
            nhit = -nhit;
        }
        nhit
    }

    fn intersect(&self, ray: &Ray) -> (f32, f32) {
        let (t0, t1) = (INFINITY, INFINITY);
        let edge_0 = self.b - self.a;
        let edge_1 = self.c - self.a;
        let h = ray.dir.cross(&edge_1);

        let dot = edge_0.dot(&h);
        if dot == 0. {
            return (t0, t1);
        }

        let dot_r = 1. / dot;
        let s = ray.origin - self.a;
        let u = dot_r * s.dot(&h);
        if u < 0. || u > 1. {
            return (t0, t1);
        }

        let cross = s.cross(&edge_0);
        let v = dot_r * ray.dir.dot(&cross);
        if v < 0. || v + u > 1. {
            return (t0, t1);
        }

        let hit = dot_r * edge_1.dot(&cross);
        if hit > 0. {
            (hit, t1)
        } else {
            (t0, t1)
        }
    }
}

impl SetPosition for Triangle {
    fn set_position(&mut self, _pos: Vec3f) {
    }

    fn get_position(&self) -> Vec3f {
        Vec3::new(0., 0., 0.)
    }
}
