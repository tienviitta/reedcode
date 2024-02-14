use std::collections::LinkedList;

mod add_two_num_std;
mod add_two_numbers;
mod two_sum;

fn main() {
    println!("Leetcode Blind 75:");
    let nums = vec![2, 4, 9, 6, 5];
    let target = 10;
    let c = two_sum::two_sum(nums, target);
    println!("  #1: Two Sum: {:?}", c);
    let vec0: Vec<i32> = vec![0, 1, 2, 3];
    let vec1: Vec<i32> = vec![4, 5, 6, 7, 9, 1];
    let lila = add_two_numbers::add_two_numbers(
        add_two_numbers::to_list(vec0),
        add_two_numbers::to_list(vec1),
    );
    println!(
        "  #2a: Add Two Numbers: {:?}",
        add_two_numbers::to_vec(lila)
    );
    let lil0 = LinkedList::from([0, 1, 2, 3]);
    let lil1 = LinkedList::from([4, 5, 6, 7, 9, 1]);
    let lilb = add_two_num_std::add_two_num_std(lil0, lil1);
    println!("  #2b: Add Two Numbers: {:?}", lilb);
}
