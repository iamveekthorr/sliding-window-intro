fn main() {
    let nums = [3, 1, 2, 7, 4, 2, 1, 1, 5];
    let k = 8;

    let longest_sum = longest_sum_of_sub_arrays(&nums, &k);

    println!(
        "The longest sum of a subarray where actual sum equal {} is: {}",
        k, longest_sum
    );
}

fn longest_sum_of_sub_arrays(nums: &[i32], k: &i32) -> usize {
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

        ans = ans.max(right - left + 1);
    }

    ans
}
