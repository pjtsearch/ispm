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

impl std::string::ToString for ShCmd {
    fn to_string(&self) -> String {
        self.command.clone()
    }
}

impl From<Vec<ShCmd>> for ShCmd {
    fn from(commands:Vec<ShCmd>) -> ShCmd {
        let command = commands.iter()
            .map(|cmd|cmd.command.clone()).collect::<Vec<String>>()
            .join(";\n");
        ShCmd {command}
    }
}

impl From<Vec<&str>> for ShCmd {
    fn from(commands:Vec<&str>) -> ShCmd {
        let command = commands.join(";\n");
        ShCmd {command}
    }
}

impl ShCmd {
    pub fn new(command:String) -> ShCmd {
        ShCmd {command}
    }
}