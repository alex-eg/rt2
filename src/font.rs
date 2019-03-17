use crate::resource::ResourceLoader;

use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct FontManager {
    ttf: Sdl2TtfContext,
}

impl FontManager {
    pub fn new() -> FontManager {
        FontManager { ttf: sdl2::ttf::init().unwrap() }
    }

    pub fn load(&self, loader: &ResourceLoader, path: &str, size: u16) -> Font {
        let resolved_path = loader.resolve_path(path);
        self.ttf.load_font(resolved_path.as_path(), size).unwrap()
    }
}
