use ipsm_lib::source::{Source,SourceVariant};
use crate::utils::if_some::if_some;
use ipsm_lib::pkg::Pkg;
use crate::utils::find_pkgbuild::find_pkgbuild;
use ipsm_lib::shcmd::ShCmd;
use std::path::PathBuf;

pub trait PkgYamlExt {
    fn from_yaml(yaml:&yaml_rust::Yaml) -> Pkg;
}

impl PkgYamlExt for Pkg {
    fn from_yaml(yaml:&yaml_rust::Yaml) -> Pkg {
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