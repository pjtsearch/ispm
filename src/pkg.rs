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
        cmds.push(ShCmd::new(format!("rm -rf ./*")));
        cmds.push(ShCmd::new(format!("wget -O ./src.archive {}",self.url)));
        match &self.variant {
            SourceVariant::TAR => 
                cmds.push(ShCmd::new("tar -xf ./src.archive --one-top-level=./src  --strip-components=1".to_string()))
        };
        ShCmd::from(cmds).run(dir)
    }
}

pub struct Pkg {
    pub source: Source,
    pub version: String,
    pub pre_source: Option<ShCmd>,
    pub build: ShCmd,
    pub uninstall: Option<ShCmd>,
}

impl Pkg {
    pub fn install(&mut self,working_dir:&str) -> Result<(), RunErr>{
        self.source.download(working_dir.to_string())?;
        self.build.run(format!("{}/src",working_dir))?;
        Ok(())
    }
}