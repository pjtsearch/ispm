extern crate yaml_rust;
extern crate clap;
extern crate jfs;
#[macro_use]
extern crate serde_derive;
use crate::lib::source::{Source,SourceVariant};
use crate::utils::if_some::if_some;
use env_logger::Env;
use clap::{App, load_yaml};
use std::path::PathBuf;
use crate::lib::pkg::Pkg;
use crate::lib::pkgregistry::PkgRegistry;
use yaml_rust::{YamlLoader};
use std::fs;
mod utils;
mod lib;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let cli = load_yaml!("./cli.yaml");
    let matches = App::from_yaml(cli).get_matches();
    let input = PathBuf::from(matches.value_of("INPUT").unwrap_or("./pkg.yaml"));

    let yaml_str:&str = &fs::read_to_string(input).expect("Could not read pkg.yaml");
    let yaml = &YamlLoader::load_from_str(yaml_str).expect("Could not parse pkg")[0];
    let mut pkg = Pkg::from(yaml);
    if let Some(matches) = matches.subcommand_matches("install") {
        let working_dir = PathBuf::from(matches.value_of("working_dir").unwrap_or("./tmp"));
        let registry = PkgRegistry::new(PathBuf::from(matches.value_of("registry_dir").unwrap_or("./registry")));

        pkg.install(working_dir,registry).expect("installation failed");
    }
    if let Some(_matches) = matches.subcommand_matches("uninstall") {
        let registry = PkgRegistry::new(PathBuf::from(matches.value_of("registry_dir").unwrap_or("./registry")));

        pkg.uninstall(registry).expect("un-installation failed");
    }
}

use crate::utils::find_pkgbuild::find_pkgbuild;
use crate::lib::shcmd::ShCmd;

impl From<&yaml_rust::Yaml> for Pkg {
    fn from(yaml:&yaml_rust::Yaml) -> Pkg {
        let mut pkg_obj = Pkg::default();
        let name = yaml["name"].as_str();
        if_some(name,|name|{ 
            pkg_obj.with_name(name); 
        });

        let version = yaml["version"].as_str();
        if_some(version,|version|{ 
            pkg_obj.with_version(version); 
        });

        let source = yaml["source"].as_str();
        if_some(source,|source|{ 
            pkg_obj.with_source(Source {url:source.to_string(),variant:SourceVariant::TAR}); 
        });

        let deps = yaml["deps"].as_vec();
        if_some(deps,|deps|{ 
            pkg_obj.with_deps(deps
                .iter()
                .map(yaml_rust::Yaml::as_str)
                .map(Option::unwrap)
                .map(|name|find_pkgbuild(PathBuf::from("./"),name))
                .collect::<Vec<Pkg>>()); 
        });

        let pre_source = yaml["pre_source"].as_vec();
        if_some(pre_source,|pre_source|{ 
            pkg_obj.with_pre_source(ShCmd::from(pre_source.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>())); 
        });

        let build = yaml["build"].as_vec();
        if_some(build,|build|{ 
            pkg_obj.with_build(ShCmd::from(build.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>()));
        });

        let install = yaml["install"].as_vec();
        if_some(install,|install|{ 
            pkg_obj.with_install(ShCmd::from(install.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>()));
        });

        let uninstall = yaml["uninstall"].as_vec();
        if_some(uninstall,|uninstall|{ 
            pkg_obj.with_uninstall(ShCmd::from(uninstall.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>()));
        });
        pkg_obj
    }
}