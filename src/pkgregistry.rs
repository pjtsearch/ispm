use crate::traits::stackstore::StackStore;
use jfs::Store;
use std::iter::FromIterator;

pub struct PkgRegistry {
    store: Store
}

#[derive(Serialize,Deserialize,Clone,PartialEq)]
pub struct PkgReg {
    pub name:String,
    pub version:String
}

impl StackStore<PkgReg,PkgReg> for PkgRegistry {
    fn find(&self,value:PkgReg) -> Option<PkgReg> {
        let values:Vec<PkgReg> = Vec::from_iter(self.store.all::<PkgReg>().unwrap().values().cloned());
        let reg = values.iter().find(|val|**val==value).cloned();
        if let Some(reg) = reg {
            Some(reg)
        }else{
            None
        }
    }
    fn push(&self,value:PkgReg) -> std::io::Result<String>{
        self.store.save(&value)
    }
    fn delete(&self,id:&str) -> std::io::Result<()>{
        self.store.delete(id)
    }
    fn get(&self,id:&str) -> std::io::Result<PkgReg>{
        match self.store.get::<PkgReg>(id) {
            Ok(reg) => Ok(reg),
            Err(err) => Err(err)
        }
    }
}