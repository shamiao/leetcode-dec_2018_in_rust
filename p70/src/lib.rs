#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut m2, mut m1) = (0, 1);
        for _ in 0..n {
            let next_m1 = m1 + m2;
            m2 = m1;
            m1 = next_m1;
        }
        m1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use super::Solution;
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
