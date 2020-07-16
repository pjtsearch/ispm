use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;
use std::process::Command;

pub struct ShCmd {
    pub command: String
}

impl Runnable for ShCmd {
    fn run(&mut self) -> Result<(), RunErr> {
        let output = Command::new("sh").arg("-c").arg(self.command.clone()).output();
        println!("{:?}",output);
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