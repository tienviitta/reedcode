use std::collections::LinkedList;

/*
NOTE: It is almost always better to use Vec or VecDeque because array-based
containers are generally faster, more memory efficient, and make better use
of CPU cache. [https://doc.rust-lang.org/std/collections/struct.LinkedList.html]
 */
pub fn add_two_num_std(mut l0: LinkedList<i32>, mut l1: LinkedList<i32>) -> LinkedList<i32> {
    let mut lili = LinkedList::new();
    let mut carry = 0;
    while !l0.is_empty() || !l1.is_empty() || carry != 0 {
        let mut sum = carry;
        match l0.pop_front() {
            Some(node) => sum += node,
            None => (),
        }
        match l1.pop_front() {
            Some(node) => sum += node,
            None => (),
        }
        carry = sum / 10;
        lili.push_back(sum % 10);
    }
    lili
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_two_a() {
        let lil0 = LinkedList::from([0, 1, 2, 3]);
        let lil1 = LinkedList::from([4, 5, 6, 7, 9, 1]);
        let suma = LinkedList::from([4, 6, 8, 0, 0, 2]);
        assert_eq!(add_two_num_std(lil0, lil1), suma);
    }
}
