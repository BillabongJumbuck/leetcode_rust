use crate::list::singly_linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::help(l1, l2, false)
    }

    fn help(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: bool) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => {
                if carry { Some(Box::new(ListNode::new(1)))}
                else { None }
            }
            (Some(mut l1), None) => {
                let val = if carry { l1.val + 1 } else { l1.val };
                let next_carry = val >= 10;
                let mut ret = ListNode::new(val % 10);
                ret.next = Self::help(l1.next.take(), None, next_carry);
                Some(Box::new(ret))
            }
            (None, Some(mut l2)) => {
                let val = if carry { l2.val + 1 } else { l2.val };
                let next_carry = val >= 10;
                let mut ret = ListNode::new(val % 10);
                ret.next = Self::help(None, l2.next.take(), next_carry);
                Some(Box::new(ret))
            }
            (Some(mut l1), Some(mut l2)) => {
                let val = if carry { l1.val + l2.val + 1 } else { l1.val + l2.val};
                let next_carry = val >= 10;
                let mut ret = ListNode::new(val % 10);
                ret.next = Self::help(l1.next.take(), l2.next.take(), next_carry);
                Some(Box::new(ret))
            }
        }
    }
}