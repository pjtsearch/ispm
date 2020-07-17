extern crate yaml_rust;
use crate::shcmd::ShCmd;
use crate::pkg::Source;
use crate::pkg::SourceVariant;
use crate::pkg::Pkg;
use yaml_rust::{YamlLoader};
use std::fs;
mod pkg;
mod shcmd;
mod traits;
mod utils;

fn main() {
    let pkg_str:&str = &fs::read_to_string("./pkg.yaml").expect("Could not read pkg.yaml");
    let pkg = &YamlLoader::load_from_str(pkg_str).expect("Could not parse pkg")[0];
    let pre_source = pkg["pre_source"].as_vec();
    let source = pkg["source"].as_str().expect("source not found, and source is a required field");
    let version = pkg["version"].as_str().expect("version not found, and version is a required field");
    let build = pkg["build"].as_vec().expect("build not found, and build is a required field");
    let uninstall = pkg["uninstall"].as_vec();
    Pkg {
        source:Source {url:source.to_string(),variant:SourceVariant::TAR},
        build: ShCmd::from(build.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>()),
        // FIXME: Only if exists -----------------v
        pre_source: Some(ShCmd::from(pre_source.unwrap().iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>())),
        // FIXME: Only if exists -----------------v
        uninstall: Some(ShCmd::from(uninstall.unwrap().iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>())),
        version:version.to_string()
    }.install("./tmp").unwrap();
}