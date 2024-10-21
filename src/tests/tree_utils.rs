use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub(super) type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct TreeWrapper(Tree);

pub(super) fn new_tree(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

impl FromIterator<Option<i32>> for TreeWrapper {
    fn from_iter<T: IntoIterator<Item = Option<i32>>>(iter: T) -> Self {
        let nums: Vec<Option<i32>> = iter.into_iter().collect();

        if nums.is_empty() {
            TreeWrapper(None)
        } else {
            let nodes: Vec<_> = nums
                .iter()
                .map(|num| {
                    num.as_ref()
                        .map(|&v| Rc::new(RefCell::new(TreeNode::new(v))))
                })
                .collect();
            let mut kids = nodes.iter();
            let root = kids
                .next()
                .expect("No root node in a non-empty node list")
                .clone();
            for node in nodes.iter().flatten() {
                if let Some(kid) = kids.next() {
                    node.as_ref().borrow_mut().left = kid.clone();
                } else {
                    break;
                }
                if let Some(kid) = kids.next() {
                    node.as_ref().borrow_mut().right = kid.clone();
                } else {
                    break;
                }
            }
            TreeWrapper(root)
        }
    }
}

impl From<TreeWrapper> for Tree {
    fn from(wrapper: TreeWrapper) -> Self {
        wrapper.0
    }
}

#[allow(dead_code)]
pub(super) fn unwrap_tree<T: IntoIterator<Item = Option<i32>>>(iter: T) -> Tree {
    TreeWrapper::from_iter(iter).into()
}

mod tests {

    #[test]
    fn test_tree_utils() {
        use super::*;
        assert_eq!(
            TreeWrapper::from_iter(vec![Some(1), None, Some(2), Some(3)]),
            TreeWrapper(new_tree(
                1,
                None,
                new_tree(2, new_tree(3, None, None), None)
            ))
        );
        assert_eq!(
            // [5,4,7,3,null,2,null,-1,null,9]
            TreeWrapper::from_iter(vec![
                Some(5),
                Some(4),
                Some(7),
                Some(3),
                None,
                Some(2),
                None,
                Some(-1),
                None,
                Some(9)
            ]),
            TreeWrapper(new_tree(
                5,
                new_tree(4, new_tree(3, new_tree(-1, None, None), None), None),
                new_tree(7, new_tree(2, new_tree(9, None, None), None), None)
            ))
        );
    }
}
