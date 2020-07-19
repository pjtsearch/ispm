use crate::lib::pkg::Pkg;
use std::path::PathBuf;
use yaml_rust::{YamlLoader};

pub fn find_pkgbuild(search_dir:PathBuf,name:&str) -> Pkg {
    let yaml_str = &std::fs::read_to_string(
        PathBuf::from(search_dir).join(format!("{}.yaml",name))
    ).expect("could not read pkg");
    let yaml = &YamlLoader::load_from_str(yaml_str).expect("Could not parse pkg")[0];
    Pkg::from(yaml)
}