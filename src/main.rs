extern crate yaml_rust;
extern crate clap;
extern crate jfs;
#[macro_use]
extern crate serde_derive;
use ego_tree::NodeMut;
use ego_tree::NodeRef;
use crate::utils::dep_tree::dep_tree;
use env_logger::Env;
use clap::{App, load_yaml};
use std::path::PathBuf;
use crate::pkg::Pkg;
use crate::pkgregistry::PkgRegistry;
use yaml_rust::{YamlLoader};
use std::fs;
mod pkg;
mod pkgregistry;
mod shcmd;
mod traits;
mod utils;
mod source;

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