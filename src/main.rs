extern crate nalgebra as na;
extern crate num;
extern crate num_cpus;
extern crate scoped_threadpool;
extern crate sdl2;
extern crate time;

mod camera;
mod fps_counter;
mod geometry;
mod light;
mod material;
mod object;
mod raytracer;
mod surface;
mod input;
mod resource;
mod animation;

use camera::CamBuilder;
use fps_counter::FpsCounter;
use light::Light;
use material::Material;
use object::{new_sphere, new_box, new_triangle, new_square, Object, BoxBuilder};
use raytracer::march;
use input::InputHandler;
use resource::ResourceLoader;
use animation::Animation;

use na::Vector3 as Vec3;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

use std::time::Duration;
use std::thread;

const CAM_WIDTH: u32 = 640;
const CAM_HEIGHT: u32 = 480;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let res = ResourceLoader::new();
    let font = res.load_font("~res:fonts/courier_code.ttf", 16);
    println!("Num of cpus: {}", num_cpus::get());
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

    let red = Material { ambient: Vec3::new(0.1, 0.1, 0.1),
                         diffuse: Vec3::new(1., 0., 0.),
                         specular: Vec3::new(1., 1., 1.),
                         shininess: 30.0,
                         reflection: 0.8,
                         refraction: 0. };

    let blue = Material { ambient: Vec3::new(0.1, 0.1, 0.1),
                          diffuse: Vec3::new(0., 0.3, 1.),
                          specular: Vec3::new(1., 1., 1.),
                          shininess: 10.0,
                          reflection: 0.4,
                          refraction: 1.025 };

    let green = Material { ambient: Vec3::new(0.1, 0.1, 0.1),
                           diffuse: Vec3::new(0., 1., 0.3),
                           specular: Vec3::new(1., 1., 1.),
                           shininess: 10.0,
                           reflection: 0.8,
                           refraction: 0. };

    let mut camera = CamBuilder::new()
        .eye(Vec3::new(0., 0., 60.))
        .center(Vec3::new(0., 0., 59.))
        .fov(30.)
        .width(CAM_WIDTH)
        .height(CAM_HEIGHT)
        .up(Vec3::new(0., -1., 0.))
        .build();

    let sphere1 = new_sphere("s1", Vec3::new(15., 15., 15.),
                             5., green);
    let box1 = new_box("b1", Vec3::new(10., 10., -20.), Vec3::new(20., 20., -10.), red);
    let sphere3 = new_sphere("s5", Vec3::new(-15., 15., 15.),
                             5., blue);
    let sphere4 = new_sphere("s6", Vec3::new(-15., 15., -15.),
                             5., red);
    let sphere5 = new_sphere("s3", Vec3::new(15., -15., 15.),
                             5., green);
    let sphere6 = new_sphere("s4", Vec3::new(15., -15., -15.),
                             5., blue);
    let sphere7 = new_sphere("s7", Vec3::new(-15., -15., 15.),
                             5., red);
    let sphere8 = new_sphere("s8", Vec3::new(-15., -15., -15.),
                             5., green);
    let mut a1= Animation::new(&sphere1, vec![
        Vec3::new(0., -10., 0.)
    ]);
    let mut a2= Animation::new(&box1, vec![
        Vec3::new(0., 7., 0.)
    ]);
    let mut a3= Animation::new(&sphere3, vec![
        Vec3::new(0., -8., 0.)
    ]);
    let mut a4= Animation::new(&sphere4, vec![
        Vec3::new(0., 5., 0.)
    ]);

    let triangle = new_triangle("tri1", Vec3::new(0., -10., -10.),
                                Vec3::new(-10., 10., 0.),
                                Vec3::new(10., 10., 0.),
                                blue);
    let small_tree = BoxBuilder::new()
        .add(10, 20, 0, 1)
        .add(10, 19, 0, 1)
        .add(10, 18, 1, 1)
        .add(10, 17, 1, 1)
        .add(10, 16, 1, 1)
        .build("tree", green);
    let mut objects: Vec<Object> = vec![
        sphere1,
        box1,
        sphere3,
        sphere4,
        sphere5,
        sphere6,
        sphere7,
        triangle,
        small_tree,
        sphere8,
        new_square("square", Vec3::new(0., 30., 0.), 128, blue),
    ];

    let light1 = Light { pos: Vec3::new(0., -60., 0.),
                         color: Vec3::new(1., 1., 1.) };
    let lights: Vec<Light> = vec![
        light1,
    ];

    canvas.clear();
    canvas.present();

    let mut pump = context.event_pump().unwrap();

    let mut fps = FpsCounter::new(100);
    fps.restart();
    let mut input_handler = InputHandler::new();
    let mut first = true;
    'running: loop {
        input_handler.clear();
        for event in pump.poll_iter() {
            match event {
                Event::Quit {..}  => {
                    break 'running
                },

                _ => {
                    input_handler.process(event, &mut camera, &context)
                }
            }
        }
        input_handler.update(&mut camera);
        fps.update();
        a1.update(&mut objects[0]);
        a2.update(&mut objects[1]);
        a3.update(&mut objects[2]);
        a4.update(&mut objects[3]);
        if first || input_handler.dirty ||
            a1.dirty || a2.dirty || a3.dirty || a4.dirty {
                let updated = march(&camera, &objects, &lights);
                pixels[..updated.len()].clone_from_slice(&updated[..]);
                let _ = texture.update(None, &pixels, CAM_WIDTH as usize * 3);
                first = false;
            }
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        let white = sdl2::pixels::Color{ r: 255, g: 255, b: 255, a: 255 };
        let f_surf = font.render(&format!("FPS: {:.2}", fps.fps())).solid(white).unwrap();
        let f_rect = f_surf.rect();
        let f_tex = tex_creator.create_texture_from_surface(&f_surf).unwrap();
        canvas.copy(&f_tex, None, f_rect).unwrap();
        canvas.present();
        thread::sleep(Duration::from_millis(10));
    }
}
