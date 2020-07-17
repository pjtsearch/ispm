use crate::utils::if_some::if_some;
use crate::shcmd::ShCmd;
use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;

pub enum SourceVariant {
    TAR
}

pub struct Source {
    pub url: String,
    pub variant: SourceVariant,
}

impl Source {
    fn download(&mut self,dir:&str) -> Result<(), RunErr>{
        let mut cmds = Vec::new();
        cmds.push(ShCmd::from("rm -rf ./*"));
        cmds.push(ShCmd::from(format!("wget -O ./src.archive {}",self.url)));
        match &self.variant {
            SourceVariant::TAR => 
                cmds.push(ShCmd::from("tar -xf ./src.archive --one-top-level=./src  --strip-components=1"))
        };
        ShCmd::from(cmds).run(dir)
    }
}

#[derive(Default)]
pub struct Pkg {
    pub name: Option<String>,
    pub version: Option<String>,
    pub source: Option<Source>,
    pub pre_source: Option<ShCmd>,
    pub build: Option<ShCmd>,
    pub uninstall: Option<ShCmd>,
}

impl Pkg {
    pub fn install(&mut self,working_dir:&str) -> Result<(), RunErr>{
        self.source.as_mut().expect("source required for install").download(working_dir)?;
        self.build.as_mut().expect("build section required for install").run(&format!("{}/src",working_dir))?;
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
    pub fn with_pre_source(&mut self,pre_source:ShCmd) -> &mut Pkg {
        self.pre_source = Some(pre_source);
        self
    }
    pub fn with_build(&mut self,build:ShCmd) -> &mut Pkg {
        self.build = Some(build);
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

        let pre_source = yaml["pre_source"].as_vec();
        if_some(pre_source,|pre_source|{ 
            pkg_obj.with_pre_source(ShCmd::from(pre_source.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>())); 
        });

        let build = yaml["build"].as_vec();
        if_some(build,|build|{ 
            pkg_obj.with_build(ShCmd::from(build.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>()));
        });

        let uninstall = yaml["uninstall"].as_vec();
        if_some(uninstall,|uninstall|{ 
            pkg_obj.with_uninstall(ShCmd::from(uninstall.iter().map(yaml_rust::Yaml::as_str).map(Option::unwrap).collect::<Vec<&str>>()));
        });
        pkg_obj
    }
}