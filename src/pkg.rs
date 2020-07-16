use crate::shcmd::ShCmd;
use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;
use crate::cmdsection::CmdSection;

pub enum SourceVariant {
    TAR
}

pub struct Source {
    pub url: String,
    pub variant: SourceVariant,
}

impl Runnable for Source {
    fn run(&mut self,dir:String) -> Result<(), RunErr>{
        let mut cmds = Vec::new();
        cmds.push(ShCmd::new(format!("rm -rf ./*")));
        cmds.push(ShCmd::new(format!("wget -O ./src.archive {}",self.url)));
        match &self.variant {
            SourceVariant::TAR => 
                cmds.push(ShCmd::new("tar -xf ./src.archive --one-top-level=./src  --strip-components=1".to_string()))
        };
        CmdSection::new(cmds).run(dir)
    }
}

pub struct Pkg {
    pub source: Source,
    pub version: String,
    pub pre_source: Option<CmdSection>,
    pub build: CmdSection,
    pub uninstall: Option<CmdSection>,
}

impl Pkg {
    pub fn install(&mut self,working_dir:&str) -> Result<(), RunErr>{
        let mut results:Vec<Result<(),RunErr>> = Vec::new();
        results.push(self.source.run(working_dir.to_string()));
        results.push(self.build.run(format!("{}/src",working_dir)));
        match results.iter().find(|res|res.is_err()) {
            Some(error) => Err(error.clone().err().unwrap()),
            None => Ok(())
        }
    }
}