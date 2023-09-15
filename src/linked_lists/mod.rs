pub mod delete_duplicates;
pub mod middle_node;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl ListNode {
    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_vec() {
        let actual = ListNode::from_vec(&[1, 2]);
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        assert_eq!(actual, expected);
    }

    #[test]
    fn create_from_empty_vec() {
        let actual = ListNode::from_vec(&[]);
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn create_from_vec_with_one_element() {
        let actual = ListNode::from_vec(&[1]);
        let expected = Some(Box::new(ListNode { val: 1, next: None }));
        assert_eq!(actual, expected);
    }
}
