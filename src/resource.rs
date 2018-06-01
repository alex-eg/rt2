use sdl2::ttf::{Sdl2TtfContext, Font};
use sdl2;
use std::path::PathBuf;
use std;

pub struct ResourceLoader {
    res_prefix: String,
    res_dir: PathBuf,
    ttf: Sdl2TtfContext,
}

impl ResourceLoader {
    pub fn new() -> ResourceLoader {
        let res_dir =
            match std::env::current_exe() {
                Ok(path_buf) => {
                    match std::fs::read_link(path_buf.as_path()) {
                        Ok(mut path_buf) => {
                            path_buf.pop();
                            path_buf.push("res");
                            path_buf
                        },
                        Err(_) => PathBuf::from("./res/")
                    }
                },
                Err(_) => PathBuf::from("./res/")
            };
        let ttf = sdl2::ttf::init().unwrap();
        ResourceLoader {
            res_prefix: String::from("~res:"),
            res_dir,
            ttf
        }
    }

    fn resolve_path(&self, path: &str) -> PathBuf {
        if path.starts_with(&self.res_prefix) {
            let rel_path = path.replacen(&self.res_prefix, "", 1);
            self.res_dir.join(rel_path).canonicalize().unwrap()
        } else {
            PathBuf::from(path)
        }
    }

    pub fn load_font(&self, path: &str, size: u16) -> Font {
        let resolved_path = self.resolve_path(path);
        self.ttf.load_font(resolved_path.as_path(), size).unwrap()
    }
}
