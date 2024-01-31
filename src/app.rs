use crate::fps_counter::FpsCounter;
use crate::input::InputHandler;
use crate::raytracer::march;
use crate::scene::Scene;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

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

    let lines = std::fs::read_to_string("src/scene.ron").expect("Can't read scene.ron");
    let mut scene: Scene = ron::de::from_str(&lines).unwrap();

    let pixel_count: usize = scene.cam.width as usize * scene.cam.height as usize * 3;
    let mut pixels: Vec<u8> = vec![0; pixel_count];

    let tex_creator = canvas.texture_creator();
    let mut texture = tex_creator
        .create_texture(
            PixelFormatEnum::RGB24,
            TextureAccess::Streaming,
            scene.cam.width,
            scene.cam.height,
        )
        .unwrap();

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
            let _ = texture.update(None, &pixels, scene.cam.width as usize * 3);
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
