use crate::shcmd::ShCmd;
use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;
use crate::cmdsection::CmdSection;

pub enum SourceVariant {
    TAR
}

pub struct Source {
    url: String,
    variant: SourceVariant
}

impl Runnable for Source {
    fn run(&mut self) -> Result<(), RunErr>{
        let mut cmds = Vec::new();
        cmds.push(ShCmd::new(format!("wget -O ./src.archive {}",self.url)));
        match &self.variant {
            SourceVariant::TAR => 
                cmds.push(ShCmd::new("tar -xf ./src.archive --one-top-level=./src  --strip-components=1".to_string()))
        };
        CmdSection::new(cmds).run()
    }
}

pub struct Pkg {
    pub source: Source,
    pub version: String,
    pub pre_source: Option<CmdSection>,
    pub build: CmdSection,
    pub uninstall: Option<CmdSection>,
}

impl Runnable for Pkg {
    fn run(&mut self) -> Result<(), RunErr>{
        self.build.run()
    }
}