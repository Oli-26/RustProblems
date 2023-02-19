use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

fn child_is_less_than(parent_node: &TreeNode, child_node : Rc<RefCell<TreeNode>>) -> bool {
    let c_refcell = child_node.borrow();
    let c = &*c_refcell;

    return c.val < parent_node.val;
}

fn child_is_greater_than(parent_node: &TreeNode, child_node : Rc<RefCell<TreeNode>>) -> bool {
    let c_refcell = child_node.borrow();
    let c = &*c_refcell;

    return c.val > parent_node.val;
}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some (rc_node) => {
            let node_refcell = rc_node.borrow();
            let node = &*node_refcell;
            match node.left.clone(){
                Some (left_node) => {
                    if(!child_is_less_than(node, left_node)){
                        return false;
                    }
                },
                None => {}
            }

            match node.right.clone(){
                Some (right_node) => {
                    if(!child_is_greater_than(node, right_node)){
                        return false;
                    }
                },
                None => {}
            }
            return is_valid_bst(node.left.clone()) && is_valid_bst(node.right.clone());   
        },
        None => {
            return true;
        }
    }
}

fn main() {
    // Create a test tree
    let root = Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    }));

    // Check if the test tree is a valid binary search tree
    if is_valid_bst(Some(root)) {
        println!("The test tree is a valid binary search tree.");
    } else {
        println!("The test tree is not a valid binary search tree.");
    }
}
