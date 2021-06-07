use std::collections::HashMap;

// 思路(f()()为方法数)：f(n)(t) = f(n-1)(t+m) + f(n-1)(t-m)
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut counts_last:HashMap<i32,i32> = HashMap::new();
    counts_last.insert(0,1);
    for (_, num) in nums.iter().enumerate() {
        let mut counts_cur:HashMap<i32, i32>= HashMap::new();
        for (sum, count) in counts_last.iter() {
            // case +
            if let Some(count_cur) = counts_cur.get_mut(&(sum+num)) {
                *count_cur += count;
            } else {
                counts_cur.insert(sum+num, *count);
            }
            // case -
            if let Some(count_cur) = counts_cur.get_mut(&(sum-num)) {
                *count_cur += count;
            } else {
                counts_cur.insert(sum-num, *count);
            }
        }
        counts_last = counts_cur;
    }

    *counts_last.get(&target).unwrap_or(&0)
}

#[test]
fn find_target_sum_ways_0() {
    let nums = vec![1,1,1,1,1];
    let target = 3;

    assert_eq!(find_target_sum_ways(nums, 3), 5);
}