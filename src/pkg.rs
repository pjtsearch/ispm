use crate::utils::find_pkgbuild::find_pkgbuild;
use crate::utils::list_dir::list_dir;
use crate::utils::required::required;
use crate::source::{Source,SourceVariant};
use crate::utils::path_to_str::path_to_str;
use crate::pkgregistry::PkgReg;
use crate::pkgregistry::PkgRegistry;
use std::path::PathBuf;
use crate::utils::if_some::if_some;
use crate::shcmd::ShCmd;
use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;
use crate::traits::kvstore::KVStore;

#[derive(Default,Clone,Debug)]
pub struct Pkg {
    pub name: Option<String>,
    pub deps: Option<Vec<Pkg>>,
    pub version: Option<String>,
    pub source: Option<Source>,
    pub pre_source: Option<ShCmd>,
    pub build: Option<ShCmd>,
    pub install: Option<ShCmd>,
    pub uninstall: Option<ShCmd>,
}

impl Pkg {
    pub fn download(&mut self,working_dir:PathBuf) -> Result<(), RunErr> {
        if let Some(pre_source) = self.pre_source.as_mut() {
            pre_source
                .dir(working_dir.clone())
                .run()?;
        }
        required("source section",self.source.as_mut())
            .download(working_dir)
    }
    pub fn build(&mut self,working_dir:PathBuf) -> Result<(), RunErr> {
        self.download(working_dir.clone())?;
        required("build section",self.build.as_mut())
            .dir(working_dir.join("src"))
            .run()
    }
    pub fn install_deps(&mut self,working_dir:PathBuf,registry:PkgRegistry) -> Result<(), RunErr>{
        required("deps section",self.deps.as_mut()).iter_mut().for_each(move |dep|{
            dep.install(working_dir.clone(),registry.clone()).expect("failed installing dep");
        });
        Ok(())
    }
    pub fn install(&mut self,working_dir:PathBuf,registry:PkgRegistry) -> Result<(), RunErr>{
        //TODO: readd
        // if registry.has(required("name",self.name.clone())){
        //     info!("skipping {} because already installed",required("name",self.name.clone()));
        //     return Ok(())
        // }
        self.install_deps(working_dir.clone(),registry.clone())?;
        self.build(working_dir.clone())?;
        required("install section",self.install.as_mut())
            .env("DESTDIR",&path_to_str(working_dir.join("install")))
            .dir(working_dir.join("src"))
            .run()?;
            
        registry.set(
            required("name",self.name.clone()),
            PkgReg {
                version:required("version",self.version.clone()),
                files:list_dir(
                    working_dir.join("install")
                )?
            }
        ).expect("could not access registry");
        Ok(())
    }
    pub fn uninstall(&mut self,registry:PkgRegistry) -> Result<(), RunErr> {
        required("uninstall section",self.uninstall.as_mut())
            .dir(PathBuf::from("/"))
            .run()?;
        registry.delete(
            required("name",self.name.clone())
        ).expect("could not access registry");
        Ok(())
    }
    pub fn with_name(&mut self,name:&str) -> &mut Pkg {
        self.name = Some(name.to_string());
        self
    }
    pub fn with_version(&mut self,version:&str) -> &mut Pkg {
        self.version = Some(version.to_string());
        self
    }
    pub fn with_source(&mut self,source:Source) -> &mut Pkg {
        self.source = Some(source);
        self
    }
    pub fn with_deps(&mut self,deps:Vec<Pkg>) -> &mut Pkg {
        self.deps = Some(deps);
        self
    }
    pub fn with_pre_source(&mut self,pre_source:ShCmd) -> &mut Pkg {
        self.pre_source = Some(pre_source);
        self
    }
    pub fn with_build(&mut self,build:ShCmd) -> &mut Pkg {
        self.build = Some(build);
        self
    }
    pub fn with_install(&mut self,build:ShCmd) -> &mut Pkg {
        self.install = Some(build);
        self
    }
    pub fn with_uninstall(&mut self,uninstall:ShCmd) -> &mut Pkg {
        self.uninstall = Some(uninstall);
        self
    }
}

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