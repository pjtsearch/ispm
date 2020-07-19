use ego_tree::Tree;
use crate::utils::dep_tree::dep_tree;
use crate::utils::map_tree::map_tree;
use ipsm_lib::pkgregistry::PkgRegistry;
use ipsm_lib::pkg::Pkg;
use ipsm_lib::traits::kvstore::KVStore;

pub fn trans_sum(pkg:&Pkg,registry:&PkgRegistry) -> Tree<String> {
    let transform = |pkg:&Pkg| -> Option<String> {
        if registry.has(pkg.clone().name.unwrap()){
            None
        }else {
            Some(pkg.clone().name.unwrap())
        }
    };
    map_tree::<Pkg,String>(dep_tree(pkg.clone()),&transform)
}