use crate::fps_counter::FpsCounter;
use crate::input::InputHandler;
use crate::raytracer::march;
use crate::scene::Scene;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

pub const CAM_WIDTH: u32 = 640;
pub const CAM_HEIGHT: u32 = 480;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

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

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .unwrap();
    let tex_creator = canvas.texture_creator();
    let mut texture = tex_creator
        .create_texture(
            PixelFormatEnum::RGB24,
            TextureAccess::Streaming,
            CAM_WIDTH,
            CAM_HEIGHT,
        )
        .unwrap();
    const PIX_SIZE: usize = CAM_WIDTH as usize * CAM_HEIGHT as usize * 3;
    let mut pixels: [u8; PIX_SIZE] = [0; PIX_SIZE];

    let lines = std::fs::read_to_string("src/scene.ron").expect("Can't read scene.ron");
    let mut scene: Scene = ron::de::from_str(&lines).unwrap();

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

                _ => input_handler.process(&event, &mut scene.cam, &context),
            }
        }
        input_handler.update(&mut scene.cam);
        fps.update();
        scene.update_objects();
        if first || input_handler.dirty || scene.any_animation_dirty() {
            let updated = march(&scene);
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
