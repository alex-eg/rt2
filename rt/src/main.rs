extern crate sdl2;
extern crate nalgebra as na;
extern crate rand;
extern crate num;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureAccess;
use std::{thread};
use std::time::Duration;

use na::Vec3;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const CAM_WIDTH: u32 = 160;
const CAM_HEIGHT: u32 = 120;

mod raytracer;
mod camera;
mod geometry;
mod light;

use raytracer::march;
use camera::CamBuilder;
use geometry::{BoxBuilder};
use light::Light;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window("demo window", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut texture = renderer.create_texture(PixelFormatEnum::RGB24,
                                              TextureAccess::Static,
                                              CAM_WIDTH, CAM_HEIGHT).unwrap();
    const PIX_SIZE: usize = CAM_WIDTH as usize * CAM_HEIGHT as usize * 3;
    let mut pixels: [u8; PIX_SIZE] = [0; PIX_SIZE];

    let mut camera = CamBuilder::new()
        .eye(Vec3 { x: 0., y: 0., z: 60. })
        .center(Vec3 { x: 0., y: 0., z: 59. })
        .fov(30.)
        .width(CAM_WIDTH)
        .height(CAM_HEIGHT)
        .up(Vec3 { x: 0., y: -1., z: 0. })
        .build();

    let sphere1 = Object::Sphere { center: Vec3 { x: 20., y: 20., z: 20. },
                                   radius: 5. };
    let sphere2 = Object::Sphere { center: Vec3 { x: 20., y: 20., z: -20. },
                                   radius: 5. };
    let sphere3 = Object::Sphere { center: Vec3 { x: 20., y: -20., z: 20. },
                                   radius: 5. };
    let sphere4 = Object::Sphere { center: Vec3 { x: 20., y: -20., z: -20. },
                                   radius: 5. };
    let sphere5 = Object::Sphere { center: Vec3 { x: -20., y: 20., z: 20. },
                                   radius: 5. };
    let sphere6 = Object::Sphere { center: Vec3 { x: -20., y: 20., z: -20. },
                                   radius: 5. };
    let sphere7 = Object::Sphere { center: Vec3 { x: -20., y: -20., z: 20. },
                                   radius: 5. };
    let sphere8 = Object::Sphere { center: Vec3 { x: -20., y: -20., z: -20. },
                                   radius: 5. };

    let box1 = Object::Box { vmin: Vec3 { x: 5., y: 5., z: 5. },
                             vmax: Vec3 { x: 10., y: 10., z: 10. } };

    let mut objects: Vec<Box<Object>> = vec![
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(sphere3),
        Box::new(sphere4),
        Box::new(sphere5),
        Box::new(sphere6),
        Box::new(sphere7),
        Box::new(sphere8),
        Box::new(box1),
    ];

    let small_tree = BoxBuilder::new()
        .add(10, 20, 0, 1)
        .add(10, 19, 0, 1)
        .add(10, 18, 1, 1)
        .add(10, 17, 1, 1)
        .add(10, 16, 1, 1)
        .build();

    for b in small_tree {
        objects.push(b);
    }

    let light1 = Light { pos: Vec3 { x: 0., y: 0., z: 5. },
                         color: Vec3 { x: 1., y: 1., z: 1. } };
    let lights: Vec<Box<Light>> = vec![
        Box::new(light1),
    ];

    renderer.clear();
    renderer.present();

    let mut pump = context.event_pump().unwrap();

    'running: loop {
        for event in pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    camera.roll(-1.)
                },

                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    camera.roll(1.)
                },

                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    camera.pitch(1.)
                },

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    camera.pitch(-1.)
                },

                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    camera.yaw(-1.)
                },

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    camera.yaw(1.)
                },

                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    camera.mov_fwd(1.);
                },

                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    camera.mov_fwd(-1.);
                },

                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    camera.mov_side(-1.);
                },

                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    camera.mov_side(1.);
                },


                _ => ()
            }
        }
        let updated = march(&camera, &objects, &lights);
        let mut i = 0;
        for v in updated {
            pixels[i] = (v.x * 255.) as u8;
            pixels[i + 1] = (v.y * 255.) as u8;
            pixels[i + 2] = (v.z * 255.) as u8;
            i += 3;
        }
        let _ = texture.update(None, &pixels, CAM_WIDTH as usize * 3);
        renderer.clear();
        renderer.copy(&texture, None, None);
        renderer.present();
        thread::sleep(Duration::from_millis(10));
    }
}
