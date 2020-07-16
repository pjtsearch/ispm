use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;
use crate::shcmd::ShCmd;

pub struct CmdSection {
    pub commands:Vec<ShCmd>
}

impl Runnable for CmdSection {
    fn run(&mut self, dir:String) -> Result<(), RunErr> {
        let cmd = self.commands.iter()
            .map(|cmd|cmd.command.clone()).collect::<Vec<String>>()
            .join(";\n");

        println!("### RUNNING \n{} \n###\n",cmd);

        ShCmd::new(cmd).run(dir)
    }
}

impl CmdSection {
    pub fn new(commands:Vec<ShCmd>) -> CmdSection {
        CmdSection {commands}
    }
}