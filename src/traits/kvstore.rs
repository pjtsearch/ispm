pub trait KVStore <K,V> {
    fn has(&self,key:K) -> bool;
    fn set(&self,key:K,value:V) -> std::io::Result<String>;
    fn delete(&self,key:K) -> std::io::Result<()>;
    fn get(&self,key:K) -> std::io::Result<V>;
}