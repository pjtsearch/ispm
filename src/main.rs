extern crate yaml_rust;
use crate::cmdsection::CmdSection;
use crate::shcmd::ShCmd;
use crate::pkg::Source;
use crate::pkg::SourceVariant;
use crate::pkg::Pkg;
use yaml_rust::{YamlLoader};
use std::fs;
mod pkg;
mod cmdsection;
mod shcmd;
mod traits;
mod utils;

fn main() {
    let pkg_str:&str = &fs::read_to_string("./pkg.yaml").expect("Could not read pkg.yaml");
    let pkg = &YamlLoader::load_from_str(pkg_str).expect("Could not parse pkg")[0];
    let pre_source = pkg["pre_source"].as_vec();
    let source = pkg["source"].as_str().expect("source not found, and source is a required field");
    let build = pkg["build"].as_vec().expect("build not found, and build is a required field");
    let uninstall = pkg["uninstall"].as_vec();
    Pkg {
        source:Source {url:source.to_string(),variant:SourceVariant::TAR},
        build: CmdSection::new(
            build.clone().iter().map(|cmd|ShCmd::new(cmd.as_str().unwrap().to_string())).collect()
        ),
        pre_source: Some(CmdSection::new(
            pre_source.clone().unwrap().iter().map(|cmd|ShCmd::new(cmd.as_str().unwrap().to_string())).collect()
        )),
        uninstall: Some(CmdSection::new(
            uninstall.clone().unwrap().iter().map(|cmd|ShCmd::new(cmd.as_str().unwrap().to_string())).collect()
        )),
        version:"1".to_string()
    }.install("./tmp").unwrap();
}