use std::{cmp::max, collections::HashMap};

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut seen = HashMap::with_capacity(s.len());
    let mut left = 0;
    let mut len = 0;
    for (right, char) in s.chars().enumerate() {
        if seen.contains_key(&char) && (*seen.get(&char).unwrap()) >= left {
            left = seen.get(&char).unwrap() + 1;
        } else {
            len = max(len, right - left + 1);
        }
        seen.insert(char, right);
    }
    len as i32
}

pub fn length_of_longest_substring_lc(s: String) -> i32 {
    let mut max_len: usize = 0;

    // [1] longest substring is the one with the largest
    //    difference between positions of repeated characters;
    //    thus, we should create a storage for such positions
    let mut pos: [usize; 128] = [0; 128];

    // [2] while iterating through the string (i.e., moving
    //    the end of the sliding window), we should also
    //    update the start of the window
    let mut start: usize = 0;

    for (end, ch) in s.chars().enumerate() {
        // [3] get the position for the start of sliding window
        //    with no other occurences of 'ch' in it
        start = start.max(pos[ch as usize]);

        // [4] update maximum length
        max_len = max_len.max(end - start + 1);

        // [5] set the position to be used in [3] on next iterations
        pos[ch as usize] = end + 1;
    }

    return max_len as i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring_a() {
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring_lc(String::from("abcabcbb")), 3);
    }
    #[test]
    fn test_length_of_longest_substring_b() {
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
        assert_eq!(length_of_longest_substring_lc(String::from("bbbbb")), 1);
    }
    #[test]
    fn test_length_of_longest_substring_c() {
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
        assert_eq!(length_of_longest_substring_lc(String::from("pwwkew")), 3);
    }
}
