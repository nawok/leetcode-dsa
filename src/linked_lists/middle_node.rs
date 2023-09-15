use crate::linked_lists::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    slow.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(&[3, 4, 5]);
        assert_eq!(middle_node(input), expected);
    }

    #[test]
    fn example_2() {
        let input = ListNode::from_vec(&[1, 2, 3, 4, 5, 6]);
        let expected = ListNode::from_vec(&[4, 5, 6]);
        assert_eq!(middle_node(input), expected);
    }
}
