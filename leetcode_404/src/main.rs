use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn sum_of_left_leaves(root:Option<Rc<RefCell<TreeNode>>>)->i32{
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>,is_left:bool)->i32{
        match node{
            None => 0,
            Some(n)=>{
                let n=n.borrow();
                let is_leaf=n.left.is_none() & n.right.is_none();

                if is_leaf{
                    if is_leaf {n.val}else{0}
                }else {
                    helper(&n.left, true) + helper(&n.right,false)
                }
            }
        }
    }
    helper(&root,false)
}



fn main() {
    println!("Hello, world!");
}
