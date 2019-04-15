#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        use std::mem::swap;
        let n = triangle.len();
        let mut curr_row = Vec::<i32>::with_capacity(n);
        let mut prev_row = Vec::<i32>::with_capacity(n);
        for row_index in (0..=(n - 1)).rev() {
            let row = &triangle[row_index];
            let last_row = row_index == (n - 1);
            curr_row.clear();
            for col_index in 0..=row_index {
                curr_row.push(row[col_index] + if !last_row {
                    min(prev_row[col_index], prev_row[col_index + 1])
                } else {
                    0
                });
            }
            eprintln!("row {}, curr {:?}", row_index, &curr_row);
            swap(&mut curr_row, &mut prev_row);
        }
        prev_row[0]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        use super::Solution;
        let case = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(case), 11);
    }

    #[test]
    fn case16() {
        use super::Solution;
        let case = vec![vec![-1], vec![2, 3], vec![1, -1, -3]];
        assert_eq!(Solution::minimum_total(case), -1);
    }
}
