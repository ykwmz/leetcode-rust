use std::cmp::max;

// 集合 s 包含从 1 到 n 的整数。不幸的是，因为数据错误，导致集合里面某一个数字复制了成了集合里面的另外一个数字的值，导致集合 丢失了一个数字 并且 有一个数字重复 。
// 2 <= nums.length <= 104
// 1 <= nums[i] <= 104
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut nums_ordered = vec![0;10001];
    let mut max_num = 0;
    for num in nums.iter() {
        *nums_ordered.get_mut(*num as usize).unwrap() += 1;
        max_num = max(max_num, *num);
    }

    let mut result = vec![max_num+1;2];
    for (i,&n) in nums_ordered.iter().skip(1).take((max_num) as usize).enumerate() {
        match n {
            x if x > 1 => result[0] = (i+1) as i32,
            x if x == 0 => result[1] = (i+1) as i32,
            _ => {}
        }
    }

    result
}

#[test]
fn find_error_nums_t2() {
    assert_eq!(vec![2,1], find_error_nums(vec![3,2,2]));
}

#[test]
fn find_error_nums_t1() {
    assert_eq!(vec![1,2], find_error_nums(vec![1,1]));
}

#[test]
fn find_error_nums_t0() {
    assert_eq!(vec![2,3], find_error_nums(vec![1,2,2,4]));
}