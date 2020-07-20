use ego_tree::Tree;
use crate::utils::dep_tree::dep_tree;
use crate::utils::map_tree::map_tree;
use ipsm_lib::pkgregistry::PkgRegistry;
use ipsm_lib::pkg::Pkg;
use ipsm_lib::traits::kvstore::KVStore;

pub fn trans_sum(pkg:&Pkg,registry:&PkgRegistry,has:bool) -> Tree<String> {
    let transform = |pkg:&Pkg| -> Option<String> {
        let mut res = registry.has(pkg.clone().name.unwrap());
        if !has {
            res = !res
        }
        match res {
            true => Some(pkg.clone().name.unwrap()),
            false => None
        }
    };
    map_tree::<Pkg,String>(dep_tree(pkg.clone()),&transform)
}