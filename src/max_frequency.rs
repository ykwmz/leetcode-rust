use std::collections::HashMap;
use std::cmp::max;

// 元素的 频数 是该元素在一个数组中出现的次数。
//
// 给你一个整数数组 nums 和一个整数 k 。在一步操作中，你可以选择 nums 的一个下标，并将该下标对应元素的值增加 1 。
//
// 执行最多 k 次操作后，返回数组中最高频元素的 最大可能频数 。
//
// 1 <= nums.length <= 105
// 1 <= nums[i] <= 105
// 1 <= k <= 105
pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums_ordered = nums.clone();
    nums_ordered.sort();
    let mut total = 0;
    let mut l = 0;
    let mut r = 1;
    let mut res = 1;
    while r < nums_ordered.len() {
        // 如果r和r-1存在差值，则说明已经不是连续同一个数字了，
        // 需要补齐窗口区间内的每个数字差值；如果超出k了，则把left右移直至不超出k，并计数
        total += (nums_ordered[r] - nums_ordered[r - 1]) * (r as i32 - l as i32) ;
        while total > k {
            total -= nums_ordered[r] - nums_ordered[l];
            l += 1;
        }
        res = max(res, r - l + 1);
        r += 1;
    }
    res as i32
}

pub fn max_frequency_1(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums_map:HashMap<i32, i32> = HashMap::new();
    for &num in nums.iter() {
        if let Some(n) = nums_map.get_mut(&num) {
            *n += 1;
        } else {
            nums_map.insert(num, 1);
        }
    }

    let mut nums_ordered:Vec<i32> = nums_map.keys().map(|x|*x).collect();
    nums_ordered.sort_by(|x, y|y.cmp(x));
    let mut count = 0;
    for (i, &key) in nums_ordered.iter().enumerate() {
        let mut count_cur = *nums_map.get(&key).unwrap();
        let mut k_op = k;
        let mut count_pre = count_cur;
        for &keyNext in nums_ordered.iter().skip(i+1) {
            let mut count_next = *nums_map.get(&keyNext).unwrap();
            let mut differ = key - keyNext;
            if k_op < differ {
                // 一个都不够
                break;
            }
            let mut differ_sum = differ*count_next;
            if k_op > differ_sum {
                k_op -= differ_sum;
                count_pre += count_next;
                // 有多，继续
                continue;
            } else {
                count_pre += k_op / differ;
                // k用完了，到此为止
                break;
            }
        }
        count = max(count, count_pre);
    }

    count
}

#[test]
fn max_frequency_t2() {
    // 输入：nums = [3,9,6], k = 2
    // 输出：1
    assert_eq!(1, max_frequency(vec![3,9,6], 2));
}

#[test]
fn max_frequency_t1() {
    // 输入：nums = [1,4,8,13], k = 5
    // 输出：2
    assert_eq!(2, max_frequency(vec![1,4,8,13], 5));
}

#[test]
fn max_frequency_t0() {
    // 输入：nums = [1,2,4], k = 5
    // 输出：3
    assert_eq!(3,max_frequency(vec![1,2,4], 5));
}