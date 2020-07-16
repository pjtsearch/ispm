use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;
use std::process::Command;
use std::path::Path;

pub struct ShCmd {
    pub command: String
}

impl Runnable for ShCmd {
    fn run(&mut self,dir:String) -> Result<(), RunErr> {
        let output = Command::new("sh").current_dir(Path::new(&dir)).arg("-c").arg(self.command.clone()).status();
        match output {
            Ok(_output) => Ok(()),
            Err(error) => Err(RunErr{message:error.to_string()})
        }
    }
}

impl ShCmd {
    pub fn new(command:String) -> ShCmd {
        ShCmd {command}
    }
}