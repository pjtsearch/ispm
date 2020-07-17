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