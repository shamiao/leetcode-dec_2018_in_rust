#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..(nums.len()) {
            for j in i + 1..(nums.len()) {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        unreachable!("no solution found");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let sol = super::Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(sol, vec![0, 1]);
    }

    #[test]
    #[should_panic(expected = "no solution found")]
    fn no_solution() {
        let _ = super::Solution::two_sum(vec![2, 8, 11, 15], 9);
    }
}
