use crate::smart_pointers::{BinTree, ImmBinTree};

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
}