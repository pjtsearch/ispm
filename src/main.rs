extern crate yaml_rust;
use crate::utils::if_some::if_some;
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
    let mut pkg_obj = Pkg::new();
    let pkg_str:&str = &fs::read_to_string("./pkg.yaml").expect("Could not read pkg.yaml");
    let pkg = &YamlLoader::load_from_str(pkg_str).expect("Could not parse pkg")[0];

    let name = pkg["name"].as_str();
    if_some(name,|name|{ 
        pkg_obj.with_name(name.to_string()); 
    });

    let version = pkg["version"].as_str();
    if_some(version,|version|{ 
        pkg_obj.with_version(version.to_string()); 
    });

    let source = pkg["source"].as_str();
    if_some(source,|source|{ 
        pkg_obj.with_source(Source {url:source.to_string(),variant:SourceVariant::TAR}); 
    });

    let pre_source = pkg["pre_source"].as_vec();
    if_some(pre_source,|pre_source|{ 
        pkg_obj.with_pre_source(ShCmd::from(pre_source.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>())); 
    });

    let build = pkg["build"].as_vec();
    if_some(build,|build|{ 
        pkg_obj.with_build(ShCmd::from(build.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>()));
    });

    let uninstall = pkg["uninstall"].as_vec();
    if_some(uninstall,|uninstall|{ 
        pkg_obj.with_uninstall(ShCmd::from(uninstall.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>()));
    });
    
    pkg_obj
        .install("./tmp")
        .unwrap();
}