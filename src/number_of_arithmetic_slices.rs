// 如果一个数列 至少有三个元素 ，并且任意两个相邻元素之差相同，则称该数列为等差数列。
//
// 例如，[1,3,5,7,9]、[7,7,7,7] 和 [3,-1,-5,-9] 都是等差数列。
// 给你一个整数数组 nums ，返回数组 nums 中所有为等差数组的 子数组 个数。
//
// 子数组 是数组中的一个连续序列。
//
pub fn number_of_arithmetic_slices1(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() {
        let mut differ = 0;
        let mut len = 0;
        for j in i + 1..nums.len() {
            match len {
                0 => differ = nums[j] - nums[j-1],
                l => {
                    if differ == nums[j] - nums[j-1] {
                        if l >= 1 {
                            count += 1;
                        }
                    } else {
                        break;
                    }
                }
            }
            len += 1;
        }
    }

    count
}

#[test]
fn number_of_arithmetic_slices_t1() {
    assert_eq!(0, number_of_arithmetic_slices(vec![1]));
}

#[test]
fn number_of_arithmetic_slices_t0() {
    // 输入：nums = [1,2,3,4]
    // 输出：3
    // 解释：nums 中有三个子等差数组：[1, 2, 3]、[2, 3, 4] 和 [1,2,3,4] 自身。
    assert_eq!(3, number_of_arithmetic_slices(vec![1,2,3,4]));
}