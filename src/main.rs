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

use na::Vector3 as Vec3;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const CAM_WIDTH: u32 = 160;
const CAM_HEIGHT: u32 = 120;

mod raytracer;
mod camera;
mod geometry;
mod material;
mod light;
mod object;

use raytracer::march;
use camera::CamBuilder;
use geometry::{BoxBuilder};
use material::Material;
use light::Light;
use object::{new_sphere, new_box, Object, shape_to_obect_vector};

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window("demo window", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let tex_creator = canvas.texture_creator();
    let mut texture = tex_creator.create_texture(PixelFormatEnum::RGB24,
                                              TextureAccess::Static,
                                              CAM_WIDTH, CAM_HEIGHT).unwrap();
    const PIX_SIZE: usize = CAM_WIDTH as usize * CAM_HEIGHT as usize * 3;
    let mut pixels: [u8; PIX_SIZE] = [0; PIX_SIZE];

    let red = Material::Lambert{ ambient: Vec3::new(0.1, 0.1, 0.1),
                                 diffuse: Vec3::new(1., 0., 0.),
                                 specular: Vec3::new(1., 1., 1.),
                                 emission: Vec3::new(0., 0., 0.),
                                 shininess: 30.0 };

    let blue = Material::Lambert{ ambient: Vec3::new(0.1, 0.1, 0.1),
                                  diffuse: Vec3::new(0., 0.3, 1.),
                                  specular: Vec3::new(1., 1., 1.),
                                  emission: Vec3::new(0., 0., 0.),
                                  shininess: 10.0  };

    let green = Material::Plain{ color: Vec3::new(0., 1., 0.) };

    let mut camera = CamBuilder::new()
        .eye(Vec3::new(0., 0., 60.))
        .center(Vec3::new(0., 0., 59.))
        .fov(30.)
        .width(CAM_WIDTH)
        .height(CAM_HEIGHT)
        .up(Vec3::new(0., -1., 0.))
        .build();

    let sphere1 = new_sphere(Vec3::new(20., 20., 20.),
                             5., &red);
    let sphere2 = new_sphere(Vec3::new(20., 20., -20.),
                             5., &red);
    let sphere3 = new_sphere(Vec3::new(20., -20., 20.),
                             5., &red);
    let sphere4 = new_sphere(Vec3::new(20., -20., -20.),
                             5., &red);
    let sphere5 = new_sphere(Vec3::new(-20., 20., 20.),
                             5., &green);
    let sphere6 = new_sphere(Vec3::new(-20., 20., -20.),
                             5., &green);
    let sphere7 = new_sphere(Vec3::new(-20., -20., 20.),
                             5., &red);
    let sphere8 = new_sphere(Vec3::new(-20., -20., -20.),
                             5., &red);


    let box1 = new_box(Vec3::new(5., 5., 5.),
                       Vec3::new(10., 10., 10.),
                       &blue);

    let small_tree = BoxBuilder::new()
        .add(10, 20, 0, 1)
        .add(10, 19, 0, 1)
        .add(10, 18, 1, 1)
        .add(10, 17, 1, 1)
        .add(10, 16, 1, 1)
        .build();

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

    let small_tree_shapes = shape_to_obect_vector(&small_tree, &blue);
    for b in small_tree_shapes {
        objects.push(b);
    }

    let light1 = Light { pos: Vec3::new(0., 0., 5.),
                         color: Vec3::new(1., 1., 1.) };
    let lights: Vec<Box<Light>> = vec![
        Box::new(light1),
    ];

    canvas.clear();
    canvas.present();

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
        canvas.clear();
        canvas.copy(&texture, None, None);
        canvas.present();
        thread::sleep(Duration::from_millis(10));
    }
}
