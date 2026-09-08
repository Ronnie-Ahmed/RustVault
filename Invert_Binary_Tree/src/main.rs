use std::print;

#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    fn preorder(&self) {
        print!("{} ", self.val);
        if let Some(n) = &self.left {
            n.preorder();
        }
        if let Some(n) = &self.right {
            n.preorder();
        }
    }

    fn inorder(&self) {
        if let Some(n) = &self.left {
            n.inorder();
        }
        print!("{} ", self.val);
        if let Some(n) = &self.right {
            n.inorder();
        }
    }

    fn postorder(&self) {
        if let Some(n) = &self.left {
            n.postorder();
        }
        if let Some(n) = &self.right {
            n.postorder();
        }
        print!("{} ", self.val);
    }
}

// fn preorder(node:&Option<Box<TreeNode>>){

//     if let Some(n) = node  {

//         print!("{} ",n.val);
//         preorder(&n.left);
//         preorder(&n.right);

//     }
// }

// fn inorder(node:&Option<Box<TreeNode>>){
//     if let Some(n) = node {
//         inorder(&n.left);
//         print!("{} ",n.val);
//         inorder(&n.right);
//     }
// }

// fn postorder(node:&Option<Box<TreeNode>>){
//     if let Some(n) =node  {
//         postorder(&n.left);
//         postorder(&n.right);
//         print!("{} ",n.val)
//     }
// }

fn main() {
    //    let node4=TreeNode::new(4);
    //    let node5=TreeNode::new(5);
    //    let mut node2=TreeNode::new(2);
    //    node2.left=Some(Box::new(node4));
    //    node2.right=Some(Box::new(node5));

    //    let node3=TreeNode::new(3);
    //    let mut root=TreeNode::new(1);
    //    root.left=Some(Box::new(node2));
    //    root.right=Some(Box::new(node3));
    //    println!("{:?}", root);

    let root = TreeNode::new(1);
    root.preorder();
    root.postorder();
    root.inorder();
}
