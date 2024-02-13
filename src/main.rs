mod two_sum;

fn main() {
    println!("Hello, world!");
    let nums = vec![2, 11, 7, 15];
    let target = 9;
    let c = two_sum::two_sum::two_sum(nums, target);
    println!("  two_sum: {:?}", c);
}
