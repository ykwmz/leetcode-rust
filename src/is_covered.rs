// 给你一个二维整数数组 ranges 和两个整数 left 和 right 。每个 ranges[i] = [starti, endi] 表示一个从 starti 到 endi 的 闭区间 。
//
// 如果闭区间 [left, right] 内每个整数都被 ranges 中 至少一个 区间覆盖，那么请你返回 true ，否则返回 false 。
//
// 已知区间 ranges[i] = [starti, endi] ，如果整数 x 满足 starti <= x <= endi ，那么我们称整数x 被覆盖了。
// 1 <= ranges.length <= 50
// 1 <= starti <= endi <= 50
// 1 <= left <= right <= 50
pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut range_ordered = vec![false;51];
    for range in ranges.iter() {
        for v in range[0]..=range[1] {
            *range_ordered.get_mut(v as usize).unwrap() = true;
        }
        // leetcode-cn do not support slice_fill yet
        // range_ordered[range[0] as usize..=range[1] as usize].fill(true);
    }

    range_ordered[left as usize..=right as usize].iter().all(|x|*x)
}

#[test]
fn is_covered_t1() {
    // 输入：ranges = [[1,10],[10,20]], left = 21, right = 21
    // 输出：false
    // 解释：21 没有被任何一个区间覆盖。
    assert_eq!(false, is_covered(vec![vec![1,10],vec![10,20]],21,21));
}

#[test]
fn is_covered_t0() {
    // 输入：ranges = [[1,2],[3,4],[5,6]], left = 2, right = 5
    // 输出：true
    // 解释：2 到 5 的每个整数都被覆盖了：
    // - 2 被第一个区间覆盖。
    // - 3 和 4 被第二个区间覆盖。
    // - 5 被第三个区间覆盖。
    assert_eq!(true, is_covered(vec![vec![1,2],vec![3,4],vec![5,6]],2,5));
}