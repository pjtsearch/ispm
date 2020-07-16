use std::env;
use std::path::Path;

pub fn cd (path:&str) {
    env::set_current_dir(&Path::new(path)).expect(&format!("Could not change to directory {}",path));
}