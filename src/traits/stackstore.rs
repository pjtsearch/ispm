pub trait StackStore <Res,Value> {
    fn find(&self,value:Value) -> Option<Res>;
    fn push(&self,value:Value) -> std::io::Result<String>;
    fn delete(&self,id:&str) -> std::io::Result<()>;
    fn get(&self,id:&str) -> std::io::Result<Res>;
}