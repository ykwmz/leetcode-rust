// 输入一个整型数组，数组中的一个或连续多个整数组成一个子数组。求所有子数组的和的最大值。
//
// 要求时间复杂度为O(n)。
// 1 <= arr.length <= 10^5
// -100 <= arr[i] <= 100
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = -100;
    let mut sum = max;
    for &num in nums.iter() {
        sum = if num > num + sum {
            num
        } else{
            num + sum
        };

        max = if sum > max {
            sum
        } else {
            max
        };
    }

    max
}

#[test]
fn max_sub_array_t0() {
    // 输入: nums = [-2,1,-3,4,-1,2,1,-5,4]
    // 输出: 6
    // 解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。
    assert_eq!(6, max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
}

