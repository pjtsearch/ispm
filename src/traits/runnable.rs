pub trait Runnable {
    fn run(&mut self, dir:std::path::PathBuf) -> Result<(), RunErr>;
}
#[derive(Debug, Clone)]
pub struct RunErr {
    pub message: String
}

impl From<std::io::Error> for RunErr {
    fn from(err:std::io::Error) -> RunErr{
        RunErr {message: format!("failed to run: {}",err)}
    }
}

impl std::fmt::Display for RunErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error running: {}",self.message)
    }
}