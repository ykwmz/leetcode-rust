
// 统计一个数字在排序数组中出现的次数。
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    nums.iter().filter(|&&n|n == target).count() as i32
}

#[test]
fn search_t1(){
    // 输入: nums = [5,7,7,8,8,10], target = 6
    // 输出: 0
    assert_eq!(0, search(vec![5,7,7,8,8,10], 6));
}

#[test]
fn search_t0(){
    // 输入: nums = [5,7,7,8,8,10], target = 8
    // 输出: 2
    assert_eq!(2, search(vec![5,7,7,8,8,10], 8));
}