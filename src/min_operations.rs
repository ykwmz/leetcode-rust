// 给你一个数组 target ，包含若干 互不相同 的整数，以及另一个整数数组 arr ，arr 可能 包含重复元素。
//
// 每一次操作中，你可以在 arr 的任意位置插入任一整数。比方说，如果 arr = [1,4,1,2] ，那么你可以在中间添加 3 得到 [1,4,3,1,2] 。你可以在数组最开始或最后面添加整数。
//
// 请你返回 最少 操作次数，使得 target 成为 arr 的一个子序列。
//
// 一个数组的 子序列 指的是删除原数组的某些元素（可能一个元素都不删除），同时不改变其余元素的相对顺序得到的数组。比方说，[2,7,4] 是 [4,2,3,7,2,1,4] 的子序列（加粗元素），但 [2,4,2] 不是子序列。
// 1 <= target.length, arr.length <= 10^5
// 1 <= target[i], arr[i] <= 10^9
// target 不包含任何重复元素。

use std::collections::HashMap;

// 此题思路参考：300. 最长递增子序列的官方题解
// https://leetcode-cn.com/problems/longest-increasing-subsequence/solution/zui-chang-shang-sheng-zi-xu-lie-by-leetcode-soluti/
pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
    // let mut target_numbered = vec![-1;1000000001];
    let mut target_numbered = HashMap::new();
    for (i,&t) in target.iter().enumerate() {
        if let Some(tn) = target_numbered.get_mut(&t) {
            *tn = i as i32;
        } else {
            target_numbered.insert(t, i as i32);
        }
    }

    let mut result_arr:Vec<i32> = Vec::new();
    for &a in arr.iter() {
        if let Some(&a_numbered) = target_numbered.get(&a) {
            if let Some(&last) = result_arr.last() {
                if last < a_numbered {
                    result_arr.push(a_numbered);
                } else {
                    // 插入，取代一个最小的但比它大的位置
                    match result_arr.binary_search(&a_numbered) {
                        Ok(_) => (),
                        Err(r) if r >= result_arr.len() => (),
                        Err(r) => *result_arr.get_mut(r).unwrap() = a_numbered,
                    }
                }
            } else {
                result_arr.push(a_numbered);
            }
        }
    }

    (target.len() - result_arr.len()) as i32
}

#[test]
fn min_operations_t2() {
    // 输入：target = [391381350,272779990,14679827,772485354,331478688,673799788,328776406,135016059,894557868,559131299],
    // arr = [559131299,338063085,559131299,338063085,135016059,793174916,14679827,14679827,894557868,526921048]
    // 输出：3
    assert_eq!(8, min_operations(vec![391381350,272779990,14679827,772485354,331478688,673799788,328776406,135016059,894557868,559131299],
                                 vec![559131299,338063085,559131299,338063085,135016059,793174916,14679827,14679827,894557868,526921048]));
}

#[test]
fn min_operations_t1() {
    // 输入：target = [6,4,8,1,3,2], arr = [4,7,6,2,3,8,6,1]
    // 输出：3
    assert_eq!(3, min_operations(vec![6,4,8,1,3,2], vec![4,7,6,2,3,8,6,1]));
}

#[test]
fn min_operations_t0() {
    // 输入：target = [5,1,3], arr = [9,4,2,3,4]
    // 输出：2
    // 解释：你可以添加 5 和 1 ，使得 arr 变为 [5,9,4,1,2,3,4] ，target 为 arr 的子序列。
    assert_eq!(2, min_operations(vec![5,1,3], vec![9,4,2,3,4]));
}