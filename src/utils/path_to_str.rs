use std::path::PathBuf;
use std::fs;

pub fn path_to_str(path:PathBuf) -> String {
    if !path.exists() {
        fs::create_dir_all(&path).expect("could not create dir");
    }

    fs::canonicalize(path)
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap()
}