use std::{cell::RefCell, rc::Rc};

mod solution;

#[cfg(test)]
mod tests;

pub struct Solution;

#[cfg(test)]
impl Solution {
    /// Dummy function that always return `input` as is.
    #[allow(dead_code)]
    pub(crate) fn dummy_fn<T>(input: T) -> T {
        input
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
