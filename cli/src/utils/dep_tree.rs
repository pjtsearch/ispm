use ego_tree::NodeMut;
use ipsm_lib::pkg::Pkg;
use ego_tree::Tree;

pub fn dep_tree(pkg:Pkg) -> Tree<Pkg> {
    let mut tree = ego_tree::Tree::new(pkg.clone());
    let mut root = tree.root_mut();

    fn add_pkg_to_tree(pkg:Pkg,node:&mut NodeMut<Pkg>) {
        let mut own_node = node.append(pkg.clone());
        pkg.deps.clone().unwrap().iter().for_each(|dep|{
            add_pkg_to_tree(dep.clone(),&mut own_node)
        })
    }

    pkg.deps.clone().unwrap().iter_mut().for_each(|dep|add_pkg_to_tree(dep.clone(),&mut root));

    tree
}
