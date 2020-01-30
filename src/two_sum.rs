use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map:HashMap<i32, i32> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let t = target - num;
        match map.get(&t) {
            Some(&val) => return vec![val, i as i32],
            None => {map.insert(num, i as i32);}
        }
    }
    vec![]
}

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i]+nums[j] == target{
                return vec![i as i32, j as i32]
            }
        }
    }

    vec![]
}

#[test]
fn test_two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let res = two_sum(nums, target);

    assert_eq!(res.len(), 2);
    assert_eq!(res[0], 0);
    assert_eq!(res[1], 1);

    let nums = vec![3,2,4];
    let target = 6;
    let res = two_sum(nums, target);

    assert_eq!(res.len(), 2);
    assert_eq!(res[0], 1);
    assert_eq!(res[1], 2);

    let nums = vec![3,3];
    let target = 6;
    let res = two_sum(nums, target);

    assert_eq!(res.len(), 2);
    assert_eq!(res[0], 0);
    assert_eq!(res[1], 1);
}