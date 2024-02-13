pub mod two_sum {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
}

#[cfg(test)]
mod tests {
    /*
    Example 1:
    Input: nums = [2,7,11,15], target = 9
    Output: [0,1]
    Example 2:
    Input: nums = [3,2,4], target = 6
    Output: [1,2]
    Example 3:
    Input: nums = [3,3], target = 6
    Output: [0,1]
    */
    use super::*;
    #[test]
    fn test_two_sum_a() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum::two_sum(nums, 9), [0, 1]);
    }
    #[test]
    fn test_two_sum_b() {
        let nums = vec![3, 2, 4];
        assert_eq!(two_sum::two_sum(nums, 6), [1, 2]);
    }
    #[test]
    fn test_two_sum_c() {
        let nums = vec![3, 3];
        assert_eq!(two_sum::two_sum(nums, 6), [0, 1]);
    }
}
