use crate::linked_lists::ListNode;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.as_ref()?;
    let mut head = head;
    let mut curr_node = head.as_mut().unwrap();
    while let Some(next_node) = curr_node.next.as_mut() {
        if curr_node.val == next_node.val {
            curr_node.next = next_node.next.take();
        } else {
            curr_node = curr_node.next.as_mut().unwrap();
        }
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let actual = delete_duplicates(ListNode::from_vec(&[1, 1, 2]));
        let expected = ListNode::from_vec(&[1, 2]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let actual = delete_duplicates(ListNode::from_vec(&[1, 1, 2, 3, 3]));
        let expected = ListNode::from_vec(&[1, 2, 3]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn delete_duplicates_from_empty_list() {
        let actual = delete_duplicates(ListNode::from_vec(&[]));
        let expected = ListNode::from_vec(&[]);
        assert_eq!(actual, expected);
    }
}
