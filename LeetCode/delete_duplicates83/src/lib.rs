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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(curr_node) => match curr_node.next {
            Some(next_node) => {
                if curr_node.val == next_node.val {
                    delete_duplicates(Some(next_node))
                } else {
                    let mut node = ListNode::new(curr_node.val);
                    node.next = delete_duplicates(Some(next_node));

                    Some(Box::new(node))
                }
            }
            None => Some(curr_node),
        },
        None => None,
    }
}
