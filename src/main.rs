mod animation;
mod app;
mod camera;
mod fps_counter;
mod geometry;
mod input;
mod light;
mod material;
mod object;
mod raytracer;
mod resource;
mod surface;

use nalgebra as na;
use self::na::Vector3 as Vec3;

fn main() {
    self::app::run()
}
