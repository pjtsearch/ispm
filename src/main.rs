extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;
mod pkg;
mod cmdsection;
mod shcmd;
mod traits;

fn main() {
    let pkg_str:&str = &fs::read_to_string("./pkg.yaml").expect("Could not read pkg.yaml");
    let pkg = &YamlLoader::load_from_str(pkg_str).expect("Could not parse pkg")[0];
    let pre_source = pkg["pre_source"].as_vec();
    let source = pkg["source"].as_str().expect("source not found, and source is a required field");
    let build = pkg["build"].as_vec().expect("build not found, and build is a required field");
    let uninstall = pkg["uninstall"].as_vec();
    cd("./tmp");
    match pre_source {
        Some(cmd) => run_section(cmd),
        None => ()
    }
    setup_source(source);
    cd("./src");
    run_section(build);
}

fn setup_source(source:&str) {
    run(&format!("wget -O ./src.archive {}",source));
    run(&format!("mkdir ./src"));
    run(&format!("tar -xf ./src.archive --one-top-level=./src  --strip-components=1"));
}

fn run_section(commands:&std::vec::Vec<yaml_rust::Yaml>) {
    commands.iter().for_each(|command| {
        run(&command.as_str().unwrap())
    });
}

fn run(command:&str) {
    Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect(&format!("Error running command {}",command));
}

fn cd (path:&str) {
    env::set_current_dir(&Path::new(path)).expect(&format!("Could not change to directory {}",path));
}