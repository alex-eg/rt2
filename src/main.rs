extern crate sdl2;
extern crate serde;
extern crate nalgebra;
extern crate ron;
extern crate scoped_threadpool;
extern crate num_traits;
extern crate num_cpus;

mod animation;
mod app;
mod camera;
mod fps_counter;
mod geometry;
mod input;
mod light;
mod material;
mod math;
mod object;
mod raytracer;
mod resource;
mod surface;
mod scene;
mod scene_generator;
mod font;

use nalgebra as na;

fn main() {
    self::app::run()
}
