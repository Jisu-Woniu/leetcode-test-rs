use std::ops::{Deref, DerefMut};

use crate::ListNode;

pub(super) type List = Option<Box<ListNode>>;

struct ListWrapper(List);

impl FromIterator<i32> for ListWrapper {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = i32>,
    {
        let mut head = ListNode::new(0);
        let mut p = &mut head;
        for i in iter {
            p.next = Some(Box::new(ListNode::new(i)));
            p = p.next.as_mut().unwrap();
        }
        ListWrapper(head.next)
    }
}

impl<'a> FromIterator<&'a i32> for ListWrapper {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a i32>,
    {
        let mut head = ListNode::new(0);
        let mut p = &mut head;

        for i in iter {
            p.next = Some(Box::new(ListNode::new(*i)));
            p = p.next.as_mut().unwrap();
        }

        ListWrapper(head.next)
    }
}

impl Deref for ListWrapper {
    type Target = List;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ListWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ListWrapper> for List {
    fn from(value: ListWrapper) -> Self {
        value.0
    }
}

#[allow(dead_code)]
pub(super) fn unwrap_list<I>(iter: I) -> List
where
    I: IntoIterator<Item = i32>,
{
    ListWrapper::from_iter(iter).into()
}
