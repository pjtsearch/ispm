use crate::shcmd::ShCmd;
use crate::traits::runnable::Runnable;
use crate::traits::runnable::RunErr;

pub enum SourceVariant {
    TAR
}

pub struct Source {
    pub url: String,
    pub variant: SourceVariant,
}

impl Source {
    pub fn download(&mut self,dir:std::path::PathBuf) -> Result<(), RunErr>{
        let mut cmds = Vec::new();
        cmds.push(ShCmd::from("rm -rf ./*"));
        cmds.push(ShCmd::from(format!("wget -O ./src.archive {}",self.url)));
        match &self.variant {
            SourceVariant::TAR => 
                cmds.push(ShCmd::from("tar -xf ./src.archive --one-top-level=./src  --strip-components=1"))
        };
        ShCmd::from(cmds).dir(dir).run()
    }
}