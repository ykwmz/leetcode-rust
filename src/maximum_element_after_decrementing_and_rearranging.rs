// 给你一个正整数数组 arr 。请你对 arr 执行一些操作（也可以不进行任何操作），使得数组满足以下条件：
//
// arr 中 第一个 元素必须为 1 。
// 任意相邻两个元素的差的绝对值 小于等于 1 ，也就是说，对于任意的 1 <= i < arr.length （数组下标从 0 开始），都满足 abs(arr[i] - arr[i - 1]) <= 1 。abs(x) 为 x 的绝对值。
// 你可以执行以下 2 种操作任意次：
//
// 减小 arr 中任意元素的值，使其变为一个 更小的正整数 。
// 重新排列 arr 中的元素，你可以以任意顺序重新排列。
// 请你返回执行以上操作后，在满足前文所述的条件下，arr 中可能的 最大值 。
//
// 1 <= arr.length <= 10^5
// 1 <= arr[i] <= 10^9
pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
    let mut arr_ordered = arr.to_vec();
    arr_ordered.sort();

    let mut max_num = 1;
    let mut prev = 0;
    for &cur in arr_ordered.iter() {
        match cur-prev {
            differ if differ.abs() <= 1 => prev = cur,
            differ if differ.abs() > 1 => prev = prev + 1,
            _ => {}
        }

        max_num = if prev > max_num {
            prev
        } else {
            max_num
        }
    }
    
    max_num
}

#[test]
fn maximum_element_after_decrementing_and_rearranging_t3() {
    // 输入：arr = [73,98,9]
    // 输出：3
    assert_eq!(3, maximum_element_after_decrementing_and_rearranging(vec![73,98,9]));
}

#[test]
fn maximum_element_after_decrementing_and_rearranging_t2() {
    // 输入：arr = [1,2,3,4,5]
    // 输出：5
    // 解释：数组已经满足所有条件，最大元素为 5 。
    assert_eq!(5, maximum_element_after_decrementing_and_rearranging(vec![1,2,3,4,5]));
}

#[test]
fn maximum_element_after_decrementing_and_rearranging_t1() {
    // 输入：arr = [100,1,1000]
    // 输出：3
    // 解释：
    // 一个可行的方案如下：
    // 1. 重新排列 arr 得到 [1,100,1000] 。
    // 2. 将第二个元素减小为 2 。
    // 3. 将第三个元素减小为 3 。
    // 现在 arr = [1,2,3] ，满足所有条件。
    // arr 中最大元素为 3 。
    assert_eq!(3, maximum_element_after_decrementing_and_rearranging(vec![100,1,1000]));
}

#[test]
fn maximum_element_after_decrementing_and_rearranging_t0() {
    // 输入：arr = [2,2,1,2,1]
    // 输出：2
    // 解释：
    // 我们可以重新排列 arr 得到 [1,2,2,2,1] ，该数组满足所有条件。
    // arr 中最大元素为 2 。
    assert_eq!(2, maximum_element_after_decrementing_and_rearranging(vec![2,2,1,2,1]));
}