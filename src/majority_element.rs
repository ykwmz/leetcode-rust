use std::collections::HashMap;

// 数组中占比超过一半的元素称之为主要元素。给你一个 整数 数组，找出其中的主要元素。若没有，返回 -1 。请设计时间复杂度为 O(N) 、空间复杂度为 O(1) 的解决方案。
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let major_count = (nums.len() / 2) as i32;
    let mut nums_ordered = HashMap::new();
    for &num in nums.iter() {
        if let Some(c) = nums_ordered.get_mut(&num) {
            *c += 1;
            if *c > major_count {
                return num;
            }
        } else {
            nums_ordered.insert(num, 1);
        }
    }

    -1
}

#[test]
fn majority_element_t3() {
    // 输入：[2,2,2,3,3,4,4]
    // 输出：-1
    assert_eq!(-1, majority_element(vec![2,2,2,3,3,4,4]));
}

#[test]
fn majority_element_t2() {
    // 输入：[3,2,3]
    // 输出：3
    assert_eq!(3, majority_element(vec![3,2,3]));
}

#[test]
fn majority_element_t1() {
    // 输入：[3,2]
    // 输出：-1
    assert_eq!(-1, majority_element(vec![3,2]));
}

#[test]
fn majority_element_t0() {
    // 输入：[1,2,5,9,5,9,5,5,5]
    // 输出：5
    assert_eq!(5, majority_element(vec![1,2,5,9,5,9,5,5,5]));
}