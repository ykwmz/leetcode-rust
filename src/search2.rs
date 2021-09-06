// 给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。
//
// 你可以假设 nums 中的所有元素是不重复的。
// n 将在 [1, 10000]之间。
// nums 的每个元素都将在 [-9999, 9999]之间。
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(n) => n as i32,
        _ => -1
    }
}

#[test]
fn search_t1() {
    // 输入: nums = [-1,0,3,5,9,12], target = 2
    // 输出: -1
    // 解释: 2 不存在 nums 中因此返回 -1
    assert_eq!(-1, search(vec![-1,0,3,5,9,12], 2));
}

#[test]
fn search_t0() {
    // 输入: nums = [-1,0,3,5,9,12], target = 9
    // 输出: 4
    // 解释: 9 出现在 nums 中并且下标为 4
    assert_eq!(4, search(vec![-1,0,3,5,9,12], 9));
}
