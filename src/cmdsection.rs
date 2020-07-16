use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;
use crate::shcmd::ShCmd;

pub struct CmdSection {
    pub commands:Vec<ShCmd>
}

impl Runnable for CmdSection {
    fn run(&mut self) -> Result<(), RunErr> {
        let mut results = self.commands.iter_mut().map(|cmd| {
            cmd.run()
        });
        match results.find(|result| result.is_err()) {
            Some(error) => error,
            None => Ok(())
        }
    }
}

impl CmdSection {
    pub fn new(commands:Vec<ShCmd>) -> CmdSection {
        CmdSection {commands}
    }
}