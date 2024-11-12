fn main() {
    println!("Hello, world!");
}

fn longest_sum_of_sub_arrays(nums: &[i32], k: &i32) -> i32 {
    let mut left = 0;
    let mut current = 0;
    let mut ans = 0;

    for right in 0..nums.len() {
        current += nums[right];

        // dereference k
        while current > *k {
            current -= nums[left];
            left += 1;
        }

        ans = ans.max(current)
    }

    ans
}
