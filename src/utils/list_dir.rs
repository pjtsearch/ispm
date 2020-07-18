use std::io::Error;
use std::path::PathBuf;
use std::fs::{self, DirEntry};
use std::path::Path;

fn visit_dir(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> Result<(),Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

pub fn list_dir(dir:PathBuf) -> Result<Vec<PathBuf>,Error>{
    let mut entries:Vec<PathBuf> = Vec::new();
    let mut cb = |entry:&DirEntry|{
        entries.push(entry.path())
    };
    visit_dir(&dir,&mut cb)?;
    Ok(entries)
}