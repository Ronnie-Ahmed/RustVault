// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref node) = root {
            let mut node_borrow = node.borrow_mut();
            let node=&mut *node_borrow;
            std::mem::swap(&mut node.left, &mut node.right);
            Self::invert_tree(node_borrow.left.clone());
            Self::invert_tree(node_borrow.right.clone());
        }

        root
    }
}
