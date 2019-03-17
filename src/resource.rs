use std;
use std::path::PathBuf;

pub struct ResourceLoader {
    res_prefix: String,
    res_dir: PathBuf,
}

impl ResourceLoader {
    pub fn new() -> ResourceLoader {
        let res_dir = match std::env::current_exe() {
            Ok(path_buf) => match std::fs::read_link(path_buf.as_path()) {
                Ok(mut path_buf) => {
                    path_buf.pop();
                    path_buf.push("res");
                    path_buf
                }
                Err(_) => PathBuf::from("./res/"),
            },
            Err(_) => PathBuf::from("./res/"),
        };
        ResourceLoader {
            res_prefix: String::from("~res:"),
            res_dir,
        }
    }

    pub fn resolve_path(&self, path: &str) -> PathBuf {
        if path.starts_with(&self.res_prefix) {
            let rel_path = path.replacen(&self.res_prefix, "", 1);
            self.res_dir.join(rel_path).canonicalize().unwrap()
        } else {
            PathBuf::from(path)
        }
    }
}
