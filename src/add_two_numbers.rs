// Definition for singly-linked list.
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

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut curr = None;
    for &value in vec.iter().rev() {
        let mut node = ListNode::new(value);
        node.next = curr;
        curr = Some(Box::new(node));
    }
    curr
}

pub fn to_vec(lili: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut curr = &lili;
    while curr.is_some() {
        vec.push(curr.as_ref().unwrap().val.clone());
        curr = &curr.as_ref().clone().unwrap().next;
    }
    vec
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = &l1;
    let mut l2 = &l2;
    let mut res = None;
    let mut curr = &mut res;
    let mut carry = 0;
    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;
        if let Some(node) = l1 {
            sum += node.val;
            l1 = &node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = &node.next;
        }
        carry = sum / 10;
        *curr = Some(Box::new(ListNode::new(sum % 10)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    res
}

// TODO: Unit tests?!
