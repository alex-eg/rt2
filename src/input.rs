use crate::camera::Camera;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::Sdl;

pub struct InputHandler {
    moving_left: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,

    mouse_captured: bool,
    delta: f32,
    pub dirty: bool,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            moving_left: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,

            mouse_captured: false,
            delta: 1.0,
            dirty: false,
        }
    }

    pub fn process(&mut self, event: &Event, camera: &mut Camera, context: &Sdl) {
        match event {
            Event::KeyDown {
                keycode: Some(code),
                ..
            } => match code {
                Keycode::Escape => {
                    self.mouse_captured = false;
                    context.mouse().set_relative_mouse_mode(false);
                }
                Keycode::W => {
                    self.moving_forward = true;
                }
                Keycode::S => {
                    self.moving_backward = true;
                }
                Keycode::A => {
                    self.moving_left = true;
                }
                Keycode::D => {
                    self.moving_right = true;
                }
                _ => (),
            },

            Event::KeyUp {
                keycode: Some(code),
                ..
            } => match code {
                Keycode::W => {
                    self.moving_forward = false;
                }
                Keycode::S => {
                    self.moving_backward = false;
                }
                Keycode::A => {
                    self.moving_left = false;
                }
                Keycode::D => {
                    self.moving_right = false;
                }
                _ => (),
            },

            // Mouse
            Event::MouseMotion { xrel, yrel, .. } => {
                if self.mouse_captured {
                    self.dirty = true;
                    camera.pitch(*yrel as f32 / 3.);
                    camera.yaw(*xrel as f32 / 3.);
                }
            }

            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                self.mouse_captured = true;
                context.mouse().set_relative_mouse_mode(true);
            }
            _ => (),
        }
    }

    pub fn update(&mut self, camera: &mut Camera) {
        if self.moving_left {
            camera.mov_side(-self.delta);
        }
        if self.moving_right {
            camera.mov_side(self.delta);
        }
        if self.moving_forward {
            camera.mov_fwd(self.delta);
        }
        if self.moving_backward {
            camera.mov_fwd(-self.delta);
        }

        self.dirty = self.dirty | self.moving_left
            || self.moving_right
            || self.moving_forward
            || self.moving_backward;
    }

    pub fn clear(&mut self) {
        self.dirty = false;
    }
}
