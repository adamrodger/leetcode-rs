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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (None, Some(r)) => Some(r),
        (Some(l), None) => Some(l),
        (Some(l), Some(r)) => {
            let sum = l.val + r.val;

            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: add_two_numbers(l.next, r.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: sum - 10,
                    next: add_two_numbers(
                        add_two_numbers(Some(Box::new(ListNode::new(1))), l.next), // carry over the 1
                        r.next,
                    ),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_digit_numbers() {
        // 123
        let left = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));

        // 456
        let right = Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let actual = add_two_numbers(left, right);

        // 123 + 456 = 579
        let expected = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        }));

        assert_eq!(actual, expected);
    }

    #[test]
    fn zero_total() {
        let left = Some(Box::new(ListNode::new(0)));
        let right = Some(Box::new(ListNode::new(0)));

        let actual = add_two_numbers(left, right);

        // 0 + 0 = 0
        let expected = Some(Box::new(ListNode::new(0)));

        assert_eq!(actual, expected);
    }

    #[test]
    fn ends_in_zero() {
        let left = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode::new(1))),
        }));

        let right = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode::new(2))),
        }));

        let actual = add_two_numbers(left, right);

        // 10 + 20 = 30
        let expected = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode::new(3))),
        }));

        assert_eq!(actual, expected);
    }

    #[test]
    fn digits_add_to_10_or_more() {
        let left = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode::new(5))),
        }));

        let right = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode::new(7))),
        }));

        let actual = add_two_numbers(left, right);

        // 57 + 75 = 132
        let expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));

        assert_eq!(actual, expected);
    }
}
