// 给你一个大小为 m * n 的矩阵 mat，矩阵由若干军人和平民组成，分别用 1 和 0 表示。
//
// 请你返回矩阵中战斗力最弱的 k 行的索引，按从最弱到最强排序。
//
// 如果第 i 行的军人数量少于第 j 行，或者两行军人数量相同但 i 小于 j，那么我们认为第 i 行的战斗力比第 j 行弱。
//
// 军人 总是 排在一行中的靠前位置，也就是说 1 总是出现在 0 之前。

pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut rows_counted :Vec<(usize,usize)>= mat.iter().enumerate().map(|(i,v)|
        (i, v.iter().filter(|&&n|n == 1).count())).collect();

    rows_counted.sort_by(|(_,x),(_,y)|x.cmp(y));

    rows_counted[0..k as usize].iter().map(|(k,_)|*k as i32).collect()
}

#[test]
fn k_weakest_rows_t0() {
    // 输入：mat =
    // [[1,1,0,0,0],
    //  [1,1,1,1,0],
    //  [1,0,0,0,0],
    //  [1,1,0,0,0],
    //  [1,1,1,1,1]],
    // k = 3
    // 输出：[2,0,3]
    assert_eq!(vec![2,0,3], k_weakest_rows(vec![vec![1,1,0,0,0],
    vec![1,1,1,1,0],vec![1,0,0,0,0],vec![1,1,0,0,0],vec![1,1,1,1,1]],3));
}