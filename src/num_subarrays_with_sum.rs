// 给你一个二元数组 nums ，和一个整数 goal ，请你统计并返回有多少个和为 goal 的 非空 子数组。
// 子数组 是数组的一段连续部分。
pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() {
        let mut sum = 0;
        for j in i..nums.len() {
            sum += nums[j];
            match sum {
                s if s > goal => break,
                s if s == goal => count += 1,
                _ => continue
            }
        }
    }

    count
}

#[test]
fn num_subarrays_with_sum_t1(){
    // 输入：nums = [0,0,0,0,0], goal = 0
    // 输出：15
    assert_eq!(15, num_subarrays_with_sum(vec![0,0,0,0,0], 0));
}

#[test]
fn num_subarrays_with_sum_t0(){
    // 输入：nums = [1,0,1,0,1], goal = 2
    // 输出：4
    // 解释：
    // 如下面黑体所示，有 4 个满足题目要求的子数组：
    // [1,0,1]
    // [1,0,1,0]
    // [0,1,0,1]
    // [1,0,1]
    assert_eq!(4, num_subarrays_with_sum(vec![1,0,1,0,1], 2));
}