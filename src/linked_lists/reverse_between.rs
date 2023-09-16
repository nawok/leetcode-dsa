use crate::linked_lists::ListNode;

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut before_head = Some(Box::new(ListNode { val: i32::MIN, next: head }));

    let mut before_left = &mut before_head;
    for _ in 1..left {
        before_left = &mut before_left.as_mut()?.next;
    }

    let mut prev = before_left.as_mut()?.next.take();
    let mut curr = prev.as_mut()?.next.take();
    for _ in left..right {
        let next_node = curr.as_mut()?.next.take();
        curr.as_mut()?.next = prev;
        prev = curr;
        curr = next_node;
    }

    let mut tail = &mut prev;
    for _ in left..right {
        tail = &mut tail.as_mut()?.next;
    }
    tail.as_mut()?.next = curr;
    before_left.as_mut()?.next = prev;

    before_head.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list() {
        let input = ListNode::from_vec(&[]);
        let expected = ListNode::from_vec(&[]);
        assert_eq!(reverse_between(input, 0, 0), expected);
    }

    #[test]
    fn example_1() {
        let input = ListNode::from_vec(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(&[1, 4, 3, 2, 5]);
        assert_eq!(reverse_between(input, 2, 4), expected);
    }

    #[test]
    fn example_2() {
        let input = ListNode::from_vec(&[5]);
        let expected = ListNode::from_vec(&[5]);
        assert_eq!(reverse_between(input, 1, 1), expected);
    }
}
