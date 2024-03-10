use merge_two_lists21::{merge_two_lists, ListNode};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        let mut l1: Option<Box<ListNode>> = Some(Box::(ListNode::new(1)));
        let mut l2: Option<Box<ListNode>> = ListNode::new(1);
        let test1 = vec![2, 4];
        let test2 = vec![3, 4];
        for (i, c) in test1.iter().enumerate() {
            let test = ListNode::new(*c);
            l1.next = Some(Box::new(test));
        }
        for (i, c) in test2.iter().enumerate() {
            let test = ListNode::new(*c);
            l2.next = Some(Box::new(test));
        }

        let mut l3 = ListNode::new(1);
        let test3 = vec![1, 2, 3, 4, 4];
        for (i, c) in test3.iter().enumerate() {
            let test = ListNode::new(*c);
            l3.next = Some(Box::new(test));
        }

        let result: Option<Box<ListNode>> = merge_two_lists(l1, l2);
        assert_eq!(result, l3);
    }
}
