use std::path::PathBuf;
use crate::lib::traits::kvstore::KVStore;
use jfs::Store;

#[derive(Clone)]
pub struct PkgRegistry {
    store: Store
}

#[derive(Serialize,Deserialize,Clone,PartialEq,Debug)]
pub struct PkgReg {
    pub version:String,
    pub files:Vec<PathBuf>
}

impl KVStore<String,PkgReg> for PkgRegistry {
    fn has(&self,name:String) -> bool {
        self.get(name).is_ok()
    }
    fn set(&self,name:String,reg:PkgReg) -> std::io::Result<String>{
        self.store.save_with_id(&reg, &name)
    }
    fn delete(&self,name:String) -> std::io::Result<()>{
        self.store.delete(&name)
    }
    fn get(&self,name:String) -> std::io::Result<PkgReg>{
        self.store.get::<PkgReg>(&name)
    }
}

impl PkgRegistry {
    pub fn new(path:PathBuf) -> PkgRegistry{
        PkgRegistry {store:Store::new(path).expect("could not create store")}
    }
}