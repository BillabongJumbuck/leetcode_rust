use crate::list::singly_linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut ret = head;
        let mut current = ret.as_mut().unwrap();

        while let Some(next_node) = current.next.as_mut() {
            if next_node.val == current.val {
                current.next = next_node.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        ret
    }
}
