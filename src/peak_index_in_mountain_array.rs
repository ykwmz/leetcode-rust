// 3 <= arr.length <= 104
// 0 <= arr[i] <= 106
// 题目数据保证 arr 是一个山脉数组
// 符合下列属性的数组 arr 称为 山脉数组 ：
// arr.length >= 3
// 存在 i（0 < i < arr.length - 1）使得：
// arr[0] < arr[1] < ... arr[i-1] < arr[i]
// arr[i] > arr[i+1] > ... > arr[arr.length - 1]
// 给你由整数组成的山脉数组 arr ，返回任何满足 arr[0] < arr[1] < ... arr[i - 1] < arr[i] > arr[i + 1] > ... > arr[arr.length - 1] 的下标 i 。
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    fn peak_index_in_mountain_array_sub(arr: &[i32], start: usize) -> usize {
        let pos = arr.len()/2;
        match (arr.get(pos-1), arr.get(pos), arr.get(pos+1)){
            (Some(prev),Some(cur),Some(next)) if prev < cur && cur > next =>
                start + pos,
            (Some(prev),Some(cur),Some(next)) if prev < cur && cur < next =>
                peak_index_in_mountain_array_sub(&arr[pos..], start + pos),
            (Some(prev),Some(cur),Some(next)) if prev > cur && cur > next =>
                peak_index_in_mountain_array_sub(&arr[..pos+1], start),
            _ => 0
        }
    }
    peak_index_in_mountain_array_sub(&arr[..], 0) as i32
}

#[test]
fn peak_index_in_mountain_array_t3() {
    // 输入：arr = [40,48,61,75,100,99,98,39,30,10]
    // 输出：4
    assert_eq!(4, peak_index_in_mountain_array(vec![40,48,61,75,100,99,98,39,30,10]));
}

#[test]
fn peak_index_in_mountain_array_t2() {
    // 输入：arr = [24,69,100,99,79,78,67,36,26,19]
    // 输出：2
    assert_eq!(2, peak_index_in_mountain_array(vec![24,69,100,99,79,78,67,36,26,19]));
}

#[test]
fn peak_index_in_mountain_array_t1() {
    // 输入：arr = [3,4,5,1]
    // 输出：2
    assert_eq!(2, peak_index_in_mountain_array(vec![3,4,5,1]));
}

#[test]
fn peak_index_in_mountain_array_t0() {
    // 输入：arr = [0,1,0]
    // 输出：1
    assert_eq!(1, peak_index_in_mountain_array(vec![0,1,0]));
}