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

// Leetcode 2: Add Two Numbers
pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));

        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let mut value;
        let mut curr = &mut head;
        loop {
            if carry == 0 && l1.is_none() && l2.is_none() {
                break;
            }

            value = carry;

            if l1.is_some() {
                value += l1.as_ref().unwrap().val;
                l1 = l1.unwrap().next;
            }

            if l2.is_some() {
                value += l2.as_ref().unwrap().val;
                l2 = l2.unwrap().next;
            }

            // handle carry
            carry = value / 10;
            value = value % 10;

            curr.as_mut().unwrap().next = Some(Box::new(ListNode::new(value)));
            curr = &mut curr.as_mut().unwrap().next;
        }

        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let result = Solution::add_two_numbers(l1, l2);

        let expected = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));

        assert_eq!(result, expected);
    }
}
