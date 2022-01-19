// 给你一个整数数组 nums 和一个整数 k ，判断数组中是否存在两个 不同的索引 i 和 j ，
// 满足 nums[i] == nums[j] 且 abs(i - j) <= k 。如果存在，返回 true ；否则，返回 false 。

// 1 <= nums.length <= 10^5
// -10^9 <= nums[i] <= 10^9
// 0 <= k <= 10^5

use std::collections::HashMap;

pub fn _contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    for i in 0..len {
        for j in i + 1..len {
            let dis = i as i32 -j as i32;
            if dis.abs() > k {
                break;
            } else if nums[i] == nums[j] {
                return true;
            }
        }
    }

    false
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map:HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if map.contains_key(&num) {
            let pre = map[&num];
            if (i - pre) as i32 <= k {
                return true;
            }
            if let Some(x) = map.get_mut(&num) {
                *x = i;
            }
        } else {
            map.insert(num, i);
        }
    }

    false
}

#[test]
fn contains_nearby_duplicate_t0() {
    // 输入：nums = [1,2,3,1], k = 3
    // 输出：true
    assert!(contains_nearby_duplicate(vec![1,2,3,1], 3))
}

