//给你一个正整数数组 arr ，请你计算所有可能的奇数长度子数组的和。
//
// 子数组 定义为原数组中的一个连续子序列。
//
// 请你返回 arr 中 所有奇数长度子数组的和 。
// 1 <= arr.length <= 100
// 1 <= arr[i] <= 1000
//
pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let mut sum_odd = 0;
    for i in 0..arr.len() {
        let mut sum = 0;
        for j in i..arr.len() {
            sum += arr[j];
            if (j-i)%2 == 0 {
                sum_odd += sum;
            }
        }
    }

    sum_odd
}

#[test]
fn sum_odd_length_subarrays_t2() {
    // 输入：arr = [10,11,12]
    // 输出：66
    assert_eq!(66, sum_odd_length_subarrays(vec![10,11,12]));
}

#[test]
fn sum_odd_length_subarrays_t1() {
    // 输入：arr = [1,2]
    // 输出：3
    assert_eq!(3, sum_odd_length_subarrays(vec![1,2]));
}

#[test]
fn sum_odd_length_subarrays_t0() {
    // 输入：arr = [1,4,2,5,3]
    // 输出：58
    assert_eq!(58, sum_odd_length_subarrays(vec![1,4,2,5,3]));
}