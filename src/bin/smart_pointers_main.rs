use std::rc::Rc;
use crate::smart_pointers::{BinTree, ImmBinTree, MultiOwnBinTree};

mod smart_pointers;

fn main() {
    let tree = ImmBinTree::new(
        1,
        Some(Box::new(ImmBinTree::new(
            2,
            None,
            Some(Box::new(ImmBinTree::new(
                3,
                None,
                None
            )))
        ))),
        Some(Box::new(ImmBinTree::new(
            4,
            None,
            None
        )))
    );

    println!("Tree:\n{:?}", tree);
    match tree.left() {
        None => println!("No left subtree"),
        Some(l) => println!("Left subtree:\n{:?}", l),
    }

    let multi_sub = Rc::new(MultiOwnBinTree::new(1, None, None));

    let multi_mid = Rc::new(MultiOwnBinTree::new(2, Some(Rc::clone(&multi_sub)), None));
    let multi_root = MultiOwnBinTree::new(3, Some(Rc::clone(&multi_sub)), Some(Rc::clone(&multi_mid)));

    println!("multi root:\n{:?}", multi_root);
}