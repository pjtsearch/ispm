extern crate yaml_rust;
use crate::pkg::Pkg;
use yaml_rust::{YamlLoader};
use std::fs;
mod pkg;
mod shcmd;
mod traits;
mod utils;

fn main() {
    let yaml_str:&str = &fs::read_to_string("./pkg.yaml").expect("Could not read pkg.yaml");
    let yaml = &YamlLoader::load_from_str(yaml_str).expect("Could not parse pkg")[0];
    let mut pkg = Pkg::from(yaml);
    install(&mut pkg, Some("./tmp"))
}

fn install(pkg:&mut Pkg, working_dir:Option<&str>) {
    pkg.install(working_dir.unwrap_or("./tmp")).unwrap();
}