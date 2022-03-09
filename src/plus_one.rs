// 给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一。
//
// 最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。
//
// 你可以假设除了整数 0 之外，这个整数不会以零开头。
//
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut add_one = 1;
    for d in digits.iter_mut().rev() {
        if *d + add_one < 10 {
            *d = *d + add_one;
            add_one = 0;
        } else {
            *d = *d + add_one - 10;
            add_one = 1;
        }
    }
    if add_one > 0 {
        digits.insert(0, add_one);
    }

    digits
}

#[test]
fn plus_one_t0() {
    // Input: digits = [1,2,3]
    // Output: [1,2,4]
    assert_eq!(vec![1,2,4], plus_one(vec![1,2,3]));
}

#[test]
fn plus_one_t1() {
    // Input: digits = [9]
    // Output: [1,0]
    assert_eq!(vec![1,0], plus_one(vec![9]));
}

