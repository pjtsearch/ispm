pub trait Runnable {
    fn run(&mut self, dir:String) -> Result<(), RunErr>;
}
#[derive(Debug, Clone)]
pub struct RunErr {
    pub message: String
}

impl std::fmt::Display for RunErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error running: {}",self.message)
    }
}