extern crate nalgebra as na;

struct Ray {
    dir: na::Vec3<f64>;
    origin: na::Vec3<f64>;
}

struct Intersection {
    na
}

trait Intersects<T> {
    fn intersects(T, Ray) -> Option<Intersection>;
}

struct Sphere {
    radius: f64;
    origin: na::Vec3<f64>;
}
