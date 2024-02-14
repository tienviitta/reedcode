use std::collections::HashMap;

/*
pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = Vec::new();
    for (i, val) in nums[..(nums.len() - 1)].iter().enumerate() {
        for (j, other) in nums[(i + 1)..].iter().enumerate() {
            if val + other == target {
                // res.push(i.try_into().unwrap());
                res.push(i as i32);
                // res.push((i + j + 1).try_into().unwrap());
                res.push((i + j + 1) as i32);
                return res;
            }
        }
    }
    res
}
*/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Create a hash map to store the difference between target and each number in nums
    let mut seen = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        // Check if such a difference exists in the hash map
        match seen.get(&num) {
            // If it does, return the indices of the current number and the number with the difference
            Some(&j) => return vec![j as i32, i as i32],
            // If it doesn't, add the difference between target and the current number to the hash map
            None => {
                seen.insert(target - num, i);
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum_a() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(nums, 9), [0, 1]);
    }
    #[test]
    fn test_two_sum_b() {
        let nums = vec![3, 2, 4];
        assert_eq!(two_sum(nums, 6), [1, 2]);
    }
    #[test]
    fn test_two_sum_c() {
        let nums = vec![3, 3];
        assert_eq!(two_sum(nums, 6), [0, 1]);
    }
    #[test]
    fn test_two_sum_d() {
        let nums = vec![2, 4, 9, 6, 5];
        assert_eq!(two_sum(nums, 10), [1, 3]);
    }
}
