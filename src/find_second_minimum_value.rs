// 给定一个非空特殊的二叉树，每个节点都是正数，并且每个节点的子节点数量只能为 2 或 0。如果一个节点有两个子节点的话，那么该节点的值等于两个子节点中较小的一个。
//
// 更正式地说，root.val = min(root.left.val, root.right.val) 总成立。
//
// 给出这样的一个二叉树，你需要输出所有节点中的第二小的值。如果第二小的值不存在的话，输出 -1 。

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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::min;

pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(r) = root {
        let node = r.borrow();
        return match node {
            n if n.left == None && n.right == None => -1,
            n if n.left.as_ref().unwrap().borrow().val > n.val =>
                match find_second_minimum_value(Some(n.right.as_ref().unwrap().clone())) {
                      -1 => n.left.as_ref().unwrap().borrow().val,
                      f => min(f, n.left.as_ref().unwrap().borrow().val)
                  },
            n if n.right.as_ref().unwrap().borrow().val > n.val =>
                match find_second_minimum_value(Some(n.left.as_ref().unwrap().clone())) {
                -1 => n.right.as_ref().unwrap().borrow().val,
                f => min(f, n.right.as_ref().unwrap().borrow().val)
            },
            n =>
                match (find_second_minimum_value(Some(n.left.as_ref().unwrap().clone())),
                       find_second_minimum_value(Some(n.right.as_ref().unwrap().clone()))) {
                    (-1,-1) => -1,
                    (fl,-1) => fl,
                    (-1,fr) => fr,
                    (fl,fr) => min(fl,fr),
                }
        }
    }

    -1
}

#[test]
fn find_second_minimum_value_t0() {
    // 输入：root = [2,2,5,null,null,5,7]
    // 输出：5
    // 解释：最小的值是 2 ，第二小的值是 5 。
    let tree = TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(
            TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(
            TreeNode{
                val: 5,
                left: Some(Rc::new(RefCell::new(
                    TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(
                    TreeNode::new(7)))),
            }))),
    };
    let root = Some(Rc::new(RefCell::new(tree)));
    assert_eq!(5, find_second_minimum_value(root));
}