extern crate yaml_rust;
extern crate clap;
use crate::utils::confirm::confirm;
use crate::utils::trans_sum::trans_sum;
use env_logger::Env;
use clap::{App, load_yaml};
use std::path::PathBuf;
use ipsm_lib::pkg::Pkg;
use ipsm_lib::pkgregistry::PkgRegistry;
use yaml_rust::{YamlLoader};
use std::fs;
mod utils;
use crate::utils::pkg_yaml_ext::PkgYamlExt;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let cli = load_yaml!("./cli.yaml");
    let matches = App::from_yaml(cli).get_matches();
    let input = PathBuf::from(matches.value_of("INPUT").unwrap_or("./pkg.yaml"));

    let yaml_str:&str = &fs::read_to_string(input.clone())
        .expect(&format!("Could not read {}",input.to_str().unwrap_or("unknown")));
    let yaml = &YamlLoader::load_from_str(yaml_str).expect("Could not parse pkg")[0];
    let mut pkg = Pkg::from_yaml(yaml);
    if let Some(matches) = matches.subcommand_matches("install") {
        let working_dir = PathBuf::from(matches.value_of("working_dir").unwrap_or("./tmp"));
        let registry = PkgRegistry::new(PathBuf::from(matches.value_of("registry_dir").unwrap_or("./registry")));
        println!("Summary: \n{:#?}\n",trans_sum(&pkg,&registry,false));
        if confirm("Confirm?",false).unwrap_or(false) {
            pkg.install(working_dir,registry).expect("installation failed");
        }
    }
    if let Some(_matches) = matches.subcommand_matches("uninstall") {
        let registry = PkgRegistry::new(PathBuf::from(matches.value_of("registry_dir").unwrap_or("./registry")));
        println!("Summary: \n{:#?}\n",trans_sum(&pkg,&registry,true));
        if confirm("Confirm?",false).unwrap_or(false) {
            pkg.uninstall(registry).expect("un-installation failed");
        }
    }
}