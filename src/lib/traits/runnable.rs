use std::path::PathBuf;

pub trait Runnable <Parent> {
    fn run(&mut self) -> Result<(), RunErr>;
    fn dir(&mut self, dir:PathBuf) -> &mut Parent;
    fn env(&mut self, key:&str, value:&str) -> &mut Parent;
}
#[derive(Clone)]
pub struct RunErr {
    pub message: String
}

impl From<std::io::Error> for RunErr {
    fn from(err:std::io::Error) -> RunErr{
        RunErr {message: format!("failed to run: {}",err)}
    }
}

impl From<&str> for RunErr {
    fn from(message:&str) -> RunErr{
        RunErr {message: message.to_string()}
    }
}

impl std::fmt::Display for RunErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error running: {}",self.message)
    }
}

impl std::fmt::Debug for RunErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",self.message)
    }
}