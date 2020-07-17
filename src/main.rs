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

    pkg.install("./tmp").unwrap();
}