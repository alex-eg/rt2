use crate::animation::Animation;
use crate::camera::CamBuilder;
use crate::fps_counter::FpsCounter;
use crate::input::InputHandler;
use crate::light::Light;
use crate::material::Material;
use crate::object::{new_box, new_sphere, new_square, new_triangle, Object};
use crate::raytracer::march;
use crate::scene::Scene;

use crate::math::Vec3f;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

const CAM_WIDTH: u32 = 640;
const CAM_HEIGHT: u32 = 480;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

use ron::ser::{to_string_pretty, PrettyConfig};

pub fn run() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let res = crate::resource::ResourceLoader::new();
    let font_manager = crate::font::FontManager::new();
    let font = font_manager.load(&res, "~res:fonts/courier_code.ttf", 16);
    println!("Num of cpus: {}", num_cpus::get());
    let window = video
        .window("demo window", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let tex_creator = canvas.texture_creator();
    let mut texture = tex_creator
        .create_texture(
            PixelFormatEnum::RGB24,
            TextureAccess::Static,
            CAM_WIDTH,
            CAM_HEIGHT,
        )
        .unwrap();
    const PIX_SIZE: usize = CAM_WIDTH as usize * CAM_HEIGHT as usize * 3;
    let mut pixels: [u8; PIX_SIZE] = [0; PIX_SIZE];

    let red = Material {
        ambient: Vec3f::new(0.1, 0.1, 0.1),
        diffuse: Vec3f::new(1., 0., 0.),
        specular: Vec3f::new(1., 1., 1.),
        shininess: 30.0,
        reflection: 0.8,
        refraction: 0.,
    };

    let blue = Material {
        ambient: Vec3f::new(0.1, 0.1, 0.1),
        diffuse: Vec3f::new(0., 0.3, 1.),
        specular: Vec3f::new(1., 1., 1.),
        shininess: 10.0,
        reflection: 0.4,
        refraction: 1.025,
    };

    let green = Material {
        ambient: Vec3f::new(0.1, 0.1, 0.1),
        diffuse: Vec3f::new(0., 1., 0.3),
        specular: Vec3f::new(1., 1., 1.),
        shininess: 10.0,
        reflection: 0.8,
        refraction: 0.,
    };

    let mut camera = CamBuilder::new()
        .eye(Vec3f::new(0., 0., 60.))
        .center(Vec3f::new(0., 0., 59.))
        .fov(30.)
        .width(CAM_WIDTH)
        .height(CAM_HEIGHT)
        .up(Vec3f::new(0., -1., 0.))
        .build();

    let mut sphere1 = new_sphere("s1", Vec3f::new(15., 15., 15.), 5., green);
    let mut box1 = new_box(
        "b1",
        Vec3f::new(10., 10., -20.),
        Vec3f::new(10., 10., 10.),
        red,
    );
    let mut sphere3 = new_sphere("s5", Vec3f::new(-15., 15., 15.), 5., blue);
    let mut sphere4 = new_sphere("s6", Vec3f::new(-15., 15., -15.), 5., red);
    let sphere5 = new_sphere("s3", Vec3f::new(15., -15., 15.), 5., green);
    let sphere6 = new_sphere("s4", Vec3f::new(15., -15., -15.), 5., blue);
    let sphere7 = new_sphere("s7", Vec3f::new(-15., -15., 15.), 5., red);
    let sphere8 = new_sphere("s8", Vec3f::new(-15., -15., -15.), 5., green);
    sphere1.set_animation(Animation::new(&sphere1, &[Vec3f::new(0., -10., 0.)]));
    box1.set_animation(Animation::new(&box1, &[Vec3f::new(0., 7., 0.)]));
    sphere3.set_animation(Animation::new(&sphere3, &[Vec3f::new(0., -8., 0.)]));
    sphere4.set_animation(Animation::new(&sphere4, &[Vec3f::new(0., 5., 0.)]));

    let triangle = new_triangle(
        "tri1",
        Vec3f::new(0., -10., -10.),
        Vec3f::new(-10., 10., 0.),
        Vec3f::new(10., 10., 0.),
        blue,
    );

    let objects: Vec<Object> = vec![
        sphere1,
        box1,
        sphere3,
        sphere4,
        sphere5,
        sphere6,
        sphere7,
        triangle,
        sphere8,
        new_square("square", Vec3f::new(0., 30., 0.), 128, blue),
    ];

    let light1 = Light {
        pos: Vec3f::new(0., -60., 0.),
        color: Vec3f::new(1., 1., 1.),
    };
    let lights: Vec<Light> = vec![light1];

    let mut scene: Scene = Scene{ objects: objects, lights: lights };

    let s = to_string_pretty(&scene, PrettyConfig::default()).unwrap();
    println!("{}", s);

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
                Event::Quit { .. } => break 'running,

                _ => input_handler.process(&event, &mut camera, &context),
            }
        }
        input_handler.update(&mut camera);
        fps.update();
        scene.update_objects();
        if first || input_handler.dirty || scene.any_animation_dirty() {
            let updated = march(&camera, &scene);
            pixels[..updated.len()].clone_from_slice(&updated[..]);
            let _ = texture.update(None, &pixels, CAM_WIDTH as usize * 3);
            first = false;
        }
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();

        // Text rendering
        let white = sdl2::pixels::Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        let f_surf = font
            .render(&format!("FPS: {:.2}", fps.fps()))
            .solid(white)
            .unwrap();
        let f_rect = f_surf.rect();
        let f_tex = tex_creator.create_texture_from_surface(&f_surf).unwrap();
        canvas.copy(&f_tex, None, f_rect).unwrap();
        canvas.present();
    }
}
