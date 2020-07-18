use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;
use std::process::Command;

pub struct ShCmd {
    pub command: String
}

impl Runnable for ShCmd {
    fn run(&mut self,dir:std::path::PathBuf) -> Result<(), RunErr> {
        if !dir.exists() {
            Command::new("mkdir").arg(&dir).status()?;
        }
        let output = Command::new("sh").current_dir(&dir).arg("-c").arg(self.command.clone()).status();
        match output {
            Ok(_output) => Ok(()),
            Err(error) => Err(RunErr{message:format!("failed running '{}': {}",self.command,error)})
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

impl From<&str> for ShCmd {
    fn from(command:&str) -> ShCmd {
        ShCmd {command:command.to_string()}
    }
}

impl From<String> for ShCmd {
    fn from(command:String) -> ShCmd {
        ShCmd {command}
    }
}