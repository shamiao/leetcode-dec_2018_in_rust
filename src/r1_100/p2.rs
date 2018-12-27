#![allow(dead_code)]

struct Solution {}

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    fn get_and_advance(iter: &mut Option<&Box<ListNode>>) -> i32 {
        if iter.is_some() {
            let val = iter.unwrap().val;
            let curr = iter.as_ref().unwrap();
            *iter = if curr.next.is_some() {
                Some(curr.next.as_ref().unwrap())
            } else {
                None
            };
            val
        } else {
            0
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() || l2.is_none() { unreachable!("l1 or l2 is empty"); }

        let mut sol: Option<Box<ListNode>> = None;
        let mut sol_last: Option<&mut Box<ListNode>> = None;
        let mut l1_iter: Option<&Box<ListNode>> = Some(&(l1.as_ref().unwrap()));
        let mut l2_iter: Option<&Box<ListNode>> = Some(&(l2.as_ref().unwrap()));
        let mut carry = 0;

        while l1_iter.is_some() || l2_iter.is_some() || carry > 0 {
            let mut count = carry;
            count += Solution::get_and_advance(&mut l1_iter);
            count += Solution::get_and_advance(&mut l2_iter);
            carry = count / 10;
            count %= 10;
            eprintln!("{}", count);

            let append = Box::new(ListNode { val: count, next: None });
            if let Some(x) = sol_last {
                x.next = Some(append);
                sol_last = x.next.as_mut();
            } else {
                sol = Some(append);
                sol_last = Some(sol.as_mut().unwrap());
            }
        }

        sol
    }
}

pub fn main() {
    let sol = Solution::add_two_numbers(
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: None,
                    })),
                })),
            })),
        })),
        Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: None,
                    })),
                })),
            })),
        })),
    );
    println!("{:?}", sol);
}
