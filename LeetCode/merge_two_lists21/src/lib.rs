#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (None, r) => r,
        (l, None) => l,
        (Some(mut l), Some(mut r)) => {
            if l.val <= r.val {
                l.next = merge_two_lists(l.next, Some(r));
                Some(l)
            } else {
                r.next = merge_two_lists(Some(l), r.next);
                Some(r)
            }
        }
    }
}
