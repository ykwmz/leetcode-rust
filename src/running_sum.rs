// 给你一个数组 nums 。数组「动态和」的计算公式为：runningSum[i] = sum(nums[0]…nums[i]) 。
//
// 请返回 nums 的动态和。
// 1 <= nums.length <= 1000
// -10^6 <= nums[i] <= 10^6
//
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut pre_sum = 0;
    nums.into_iter().map(|n| {
        pre_sum += n;
        pre_sum
    }).collect()
}

#[test]
fn running_sum_t2() {
    // 输入：nums = [3,1,2,10,1]
    // 输出：[3,4,6,16,17]
    assert_eq!(vec![3,4,6,16,17], running_sum(vec![3,1,2,10,1]));
}

#[test]
fn running_sum_t1() {
    // 输入：nums = [1,1,1,1,1]
    // 输出：[1,2,3,4,5]
    // 解释：动态和计算过程为 [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1] 。
    assert_eq!(vec![1,2,3,4,5], running_sum(vec![1,1,1,1,1]));
}

#[test]
fn running_sum_t0() {
    // 输入：nums = [1,2,3,4]
    // 输出：[1,3,6,10]
    // 解释：动态和计算过程为 [1, 1+2, 1+2+3, 1+2+3+4] 。
    assert_eq!(vec![1,3,6,10], running_sum(vec![1,2,3,4]));
}