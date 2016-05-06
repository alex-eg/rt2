extern crate sdl2;
extern crate nalgebra as na;
extern crate rand;
extern crate num_traits;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureAccess;
use std::{thread};
use std::time::Duration;
use na::Vec3;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

mod raytracer;
mod camera;

use raytracer::{Sphere, march};
use camera::{Camera, CamBuilder};

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
                                              WIDTH, HEIGHT).unwrap();
    let mut pixels: [u8; WIDTH as usize * HEIGHT as usize * 4] =
        [0; WIDTH as usize * HEIGHT as usize * 4];

    let camera = CamBuilder::new("main")
        .eye(Vec3 { x: 0., y: 0., z: 0.})
        .fov(30.)
        .width(WIDTH)
        .height(HEIGHT)
        .up(Vec3 { x: 0., y: -1., z: 0.})
        .build();
//    let camera = Camera { eye: Vec3 { x: 0., y: 0., z: 0.,},
//                          fov: 30.0,
//                          width: WIDTH,
//                          height: HEIGHT };

    let sphere1 = Sphere { center: Vec3 { x: 20., y: 20., z: 20. },
                           radius: 5. };
    let sphere2 = Sphere { center: Vec3 { x: 20., y: 20., z: -20. },
                           radius: 5. };
    let sphere3 = Sphere { center: Vec3 { x: 20., y: -20., z: 20. },
                           radius: 5. };
    let sphere4 = Sphere { center: Vec3 { x: 20., y: -20., z: -20. },
                           radius: 5. };
    let sphere5 = Sphere { center: Vec3 { x: -20., y: 20., z: 20. },
                           radius: 5. };
    let sphere6 = Sphere { center: Vec3 { x: -20., y: 20., z: -20. },
                           radius: 5. };
    let sphere7 = Sphere { center: Vec3 { x: 2., y: 2., z: -20. },
                           radius: 5. };
    let sphere8 = Sphere { center: Vec3 { x: 1., y: 1., z: -20. },
                           radius: 5. };
    let spheres = vec![
        &sphere1,
        &sphere2,
        &sphere3,
        &sphere4,
        &sphere5,
        &sphere6,
        &sphere7,
        &sphere8,
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
                _ => ()
            }
        }
        let updated = march(&camera, &spheres);
        let mut i = 0;
        for v in updated {
            pixels[i] = (v.x * 255.) as u8;
            pixels[i + 1] = (v.y * 255.) as u8;
            pixels[i + 2] = (v.z * 255.) as u8;
            i += 3;
        }
        let _ = texture.update(None, &pixels, WIDTH as usize * 3);
        renderer.clear();
        renderer.copy(&texture, None, None);
        renderer.present();
        thread::sleep(Duration::from_millis(100));
    }
}
