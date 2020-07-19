use ego_tree::NodeRef;
use ego_tree::NodeMut;
use crate::utils::required::required;
use crate::lib::pkg::Pkg;
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

pub fn map_tree<T,N>(old_tree:Tree<T>,transform:&dyn Fn(&T)->N) -> Tree<N> {
    fn add_to_tree<T,N>(orig_node:&mut NodeRef<T>,new_node:&mut NodeMut<N>,transform:&dyn Fn(&T)->N){
        let mut own_node = new_node.append(transform(orig_node.value()));
        orig_node.children().into_iter().for_each(|child|{
            add_to_tree(&mut child.clone(),&mut own_node,transform)
        })
    }
    let old_root = old_tree.root();
    let mut new_tree = ego_tree::Tree::new(transform(old_root.value()));
    let mut new_root = new_tree.root_mut();
    old_root.children().into_iter().for_each(|child|{
        add_to_tree::<T,N>(&mut child.clone(),&mut new_root,transform.clone())
    });
    new_tree
}