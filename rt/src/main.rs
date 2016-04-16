extern crate sdl2;
extern crate nalgebra as na;
extern crate rand;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureAccess;

const HEIGHT: u32 = 800;
const WIDTH: u32 = 600;

mod raytracer;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window("demo window", HEIGHT, WIDTH)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut texture = renderer.create_texture(PixelFormatEnum::RGB888,
                                              TextureAccess::Static,
                                              WIDTH, HEIGHT).unwrap();
    let mut pixels: [u8; WIDTH as usize * HEIGHT as usize * 3] =
        [0; WIDTH as usize * HEIGHT as usize * 3];

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
        for i in 0..pixels.len() {
            pixels[i] = rand::random::<u8>();
        }
        texture.update(None, &pixels, WIDTH as usize);
        renderer.clear();
        renderer.copy(&texture, None, None);
        renderer.present();
    }
}
