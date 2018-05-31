use camera::Camera;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::Sdl;

pub struct InputHandler {
    moving_left: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,

    rolling_ccw: bool,
    rolling_cw: bool,

    mouse_captured: bool,
    delta: f64,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            moving_left: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,

            rolling_ccw: false,
            rolling_cw: false,

            mouse_captured: false,
            delta: 1.0,
        }
    }

    pub fn process(&mut self, event: Event, camera: &mut Camera, context: &Sdl) {
        match event {
            Event::KeyDown { keycode: Some(code), .. } => {
                match code {
                    Keycode::Escape => {
                        self.mouse_captured = false;
                        context.mouse().set_relative_mouse_mode(false);
                    },
                    Keycode::Q => {
                        self.rolling_ccw = true;
                    },
                    Keycode::E => {
                        self.rolling_cw = true;
                    },
                    Keycode::W => {
                        self.moving_forward = true;
                    },
                    Keycode::S => {
                        self.moving_backward = true;
                    },
                    Keycode::A => {
                        self.moving_left = true;
                    },
                    Keycode::D => {
                        self.moving_right = true;
                    },
                    _ => ()
                }
            },

            Event::KeyUp { keycode: Some(code), .. } => {
                match code {
                    Keycode::Q => {
                        self.rolling_ccw = false;
                    },
                    Keycode::E => {
                        self.rolling_cw = false;
                    },
                    Keycode::W => {
                        self.moving_forward = false;
                    },
                    Keycode::S => {
                        self.moving_backward = false;
                    },
                    Keycode::A => {
                        self.moving_left = false;
                    },
                    Keycode::D => {
                        self.moving_right = false;
                    },
                    _ => ()
                }
            },

            // Mouse
            Event::MouseMotion { xrel, yrel, .. } => {
                if self.mouse_captured {
                    camera.pitch(yrel as f64 / 3.);
                    camera.yaw(xrel as f64 / 3.);
                }
            },

            Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                self.mouse_captured = true;
                context.mouse().set_relative_mouse_mode(true);
            },

            _ => ()
        }
    }

    pub fn update(&mut self, camera: &mut Camera) {
        if self.moving_left { camera.mov_side(-self.delta); }
        if self.moving_right { camera.mov_side(self.delta); }
        if self.moving_forward { camera.mov_fwd(self.delta); }
        if self.moving_backward { camera.mov_fwd(-self.delta); }

        if self.rolling_ccw { camera.roll(-1.4); }
        if self.rolling_cw { camera.roll(1.4); }
    }
}
