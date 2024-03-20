use delete_duplicates83::{delete_duplicates, ListNode};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        let mut l1 = ListNode::new(1);
        let l2 = ListNode::new(2);
        l1.next = Some(Box::new(l2.clone()));
        let mut node = ListNode::new(1);
        node.next = Some(Box::new(l1.clone()));

        let mut s1 = ListNode::new(1);
        let s2 = ListNode::new(2);
        s1.next = Some(Box::new(s2.clone()));

        let result: Option<Box<ListNode>> = delete_duplicates(Some(Box::new(node)));
        assert_eq!(result, Some(Box::new(s1)))
    }
}
