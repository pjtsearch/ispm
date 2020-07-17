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
    fn download(&mut self,dir:String) -> Result<(), RunErr>{
        let mut cmds = Vec::new();
        cmds.push(ShCmd::new("rm -rf ./*".to_string()));
        cmds.push(ShCmd::new(format!("wget -O ./src.archive {}",self.url)));
        match &self.variant {
            SourceVariant::TAR => 
                cmds.push(ShCmd::new("tar -xf ./src.archive --one-top-level=./src  --strip-components=1".to_string()))
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
        self.source.as_mut().expect("source required for install").download(working_dir.to_string())?;
        self.build.as_mut().expect("build section required for install").run(format!("{}/src",working_dir))?;
        Ok(())
    }
    pub fn with_name(&'_ mut self,name:String) -> &'_ mut Pkg {
        self.name = Some(name);
        self
    }
    pub fn with_version(&'_ mut self,version:String) -> &'_ mut Pkg {
        self.version = Some(version);
        self
    }
    pub fn with_source(&'_ mut self,source:Source) -> &'_ mut Pkg {
        self.source = Some(source);
        self
    }
    pub fn with_pre_source(&'_ mut self,pre_source:ShCmd) -> &'_ mut Pkg {
        self.pre_source = Some(pre_source);
        self
    }
    pub fn with_build(&'_ mut self,build:ShCmd) -> &'_ mut Pkg {
        self.build = Some(build);
        self
    }
    pub fn with_uninstall(&'_ mut self,uninstall:ShCmd) -> &'_ mut Pkg {
        self.uninstall = Some(uninstall);
        self
    }
}

impl From<&yaml_rust::Yaml> for Pkg {
    fn from(yaml:&yaml_rust::Yaml) -> Pkg {
        let mut pkg_obj = Pkg::default();
        let name = yaml["name"].as_str();
        if_some(name,|name|{ 
            pkg_obj.with_name(name.to_string()); 
        });

        let version = yaml["version"].as_str();
        if_some(version,|version|{ 
            pkg_obj.with_version(version.to_string()); 
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