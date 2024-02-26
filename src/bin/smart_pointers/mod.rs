use std::rc::Rc;

pub trait BinTree<T> {
    fn val(&self) -> &T;
    fn left(&self) -> Option<&Self>;
    fn right(&self) -> Option<&Self>;
}

/// Immutable Binary Tree: example usage of Box<T> which returns immutable reference to heap-stored
/// data
#[derive(Debug)]
pub struct ImmBinTree<T> {
    val: T,
    left: Option<Box<ImmBinTree<T>>>,
    right: Option<Box<ImmBinTree<T>>>,
}

impl<T> ImmBinTree<T> {
    pub fn new(val: T, left: Option<Box<ImmBinTree<T>>>, right: Option<Box<ImmBinTree<T>>>) -> Self {
        Self {val, left, right}
    }
}

impl<T> BinTree<T> for ImmBinTree<T> {
    fn val(&self) -> &T {
        &self.val
    }


    fn left(&self) -> Option<&ImmBinTree<T>> {
        self.left.as_ref().map(|t| t.as_ref())
    }

    fn right(&self) -> Option<&ImmBinTree<T>> {
        self.right.as_ref().map(|t| t.as_ref())
    }
}

#[derive(Debug)]
pub struct MultiOwnBinTree<T> {
    val: T,
    left: Option<Rc<MultiOwnBinTree<T>>>,
    right: Option<Rc<MultiOwnBinTree<T>>>,
}

impl<T> MultiOwnBinTree<T> {
    pub fn new(val: T, left: Option<Rc<MultiOwnBinTree<T>>>, right: Option<Rc<MultiOwnBinTree<T>>>) -> Self {
        Self {val, left, right}
    }
}

impl<T> BinTree<T> for MultiOwnBinTree<T> {
    fn val(&self) -> &T {
        &self.val
    }

    fn left(&self) -> Option<&Self> {
        self.left.as_ref().map(|t| t.as_ref())
    }

    fn right(&self) -> Option<&Self> {
        self.left.as_ref().map(|t| t.as_ref())
    }
}