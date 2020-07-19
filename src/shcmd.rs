use std::collections::HashMap;
use std::path::PathBuf;
use crate::traits::runnable::RunErr;
use crate::traits::runnable::Runnable;
use std::process::Command;
use std::fs;

#[derive(Clone,Debug)]
pub struct ShCmd {
    pub command: String,
    dir: Option<PathBuf>,
    env: HashMap<String,String>
}

impl Runnable<ShCmd> for ShCmd {
    fn run(&mut self) -> Result<(), RunErr> {
        let output = Command::new("sh")
            .current_dir(&self.dir.clone().unwrap())
            .arg("-c")
            .arg(self.command.clone())
            .envs(&self.env)
            .status();
        match output {
            Ok(_output) => Ok(()),
            Err(error) => Err(RunErr{message:format!("failed running '{}': {}",self.command,error)})
        }
    }
    fn dir(&mut self,dir:PathBuf) -> &mut ShCmd {
        if !dir.exists() {
            fs::create_dir_all(&dir).expect("could not create dir");
        }
        self.dir = Some(dir);
        self
    }

    fn env(&mut self,key:&str,value:&str) -> &mut ShCmd {
        self.env.insert(key.to_string(), value.to_string());
        self
    }
}

impl ShCmd {
    fn new(command:String) -> ShCmd {
        ShCmd {
            command,
            dir:None,
            env: HashMap::new()
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
        ShCmd::new(command)
    }
}

impl From<Vec<&str>> for ShCmd {
    fn from(commands:Vec<&str>) -> ShCmd {
        let command = commands.join(";\n");
        ShCmd::new(command)
    }
}

impl From<&str> for ShCmd {
    fn from(command:&str) -> ShCmd {
        ShCmd::new(command.to_string())
    }
}

impl From<String> for ShCmd {
    fn from(command:String) -> ShCmd {
        ShCmd::new(command)
    }
}