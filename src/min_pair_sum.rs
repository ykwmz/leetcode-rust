// 一个数对 (a,b) 的 数对和 等于 a + b 。最大数对和 是一个数对数组中最大的 数对和 。
//
// 比方说，如果我们有数对 (1,5) ，(2,3) 和 (4,4)，最大数对和 为 max(1+5, 2+3, 4+4) = max(6, 5, 8) = 8 。
// 给你一个长度为 偶数 n 的数组 nums ，请你将 nums 中的元素分成 n / 2 个数对，使得：
//
// nums 中每个元素 恰好 在 一个 数对中，且
// 最大数对和 的值 最小 。
// 请你在最优数对划分的方案下，返回最小的 最大数对和 。
// n == nums.length
// 2 <= n <= 10^5
// n 是 偶数 。
// 1 <= nums[i] <= 10^5
pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums_ordered:Vec<i32> = vec![0;100001];
    for &num in nums.iter() {
        nums_ordered[num as usize] += 1;
    }
    let mut i = 1;
    let mut j = nums_ordered.len()-1;
    let mut sum = 0;
    loop {
        while i < nums_ordered.len() && nums_ordered[i] == 0 {
            i += 1;
        }
        while j > 0 && nums_ordered[j] == 0 {
            j -= 1;
        }

        if i > j {
            break;
        } else {
            sum = if i + j > sum {
                i + j
            } else {
                sum
            };

            *nums_ordered.get_mut(i).unwrap() -= 1;
            *nums_ordered.get_mut(j).unwrap() -= 1;
        }
    }
    sum as i32
}

// pub fn min_pair_sum2(nums: Vec<i32>) -> i32 {
//     let mut nums_ordered = nums;
//     nums_ordered.sort();
//     let mut i = 0;
//     let mut j = nums_ordered.len()-1;
//     let mut sum = 0;
//     while i < j {
//         sum = if nums_ordered[i] + nums_ordered[j] > sum {
//             nums_ordered[i] + nums_ordered[j]
//         } else {
//             sum
//         };
//         i += 1;
//         j -= 1;
//     }
//     sum
// }

#[test]
fn min_pair_sum_t2() {
    // 输入：nums = [4,1,5,1,2,5,1,5,5,4]
    // 输出：8
    assert_eq!(8, min_pair_sum(vec![4,1,5,1,2,5,1,5,5,4]));
}

#[test]
fn min_pair_sum_t1() {
    // 输入：nums = [3,5,4,2,4,6]
    // 输出：8
    // 解释：数组中的元素可以分为数对 (3,5)，(4,4) 和 (6,2) 。
    // 最大数对和为 max(3+5, 4+4, 6+2) = max(8, 8, 8) = 8 。
    assert_eq!(8, min_pair_sum(vec![3,5,4,2,4,6]));
}

#[test]
fn min_pair_sum_t0() {
    // 输入：nums = [3,5,2,3]
    // 输出：7
    // 解释：数组中的元素可以分为数对 (3,3) 和 (5,2) 。
    // 最大数对和为 max(3+3, 5+2) = max(6, 7) = 7 。
    assert_eq!(7, min_pair_sum(vec![3,5,2,3]));
}