use std::i32;
use std::cmp::{min, max};

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums1_iter = 0;
    let mut nums2_iter = 0;
    let len = nums1.len() + nums2.len();
    let mut num_half = 0.0;
    let mut num_half_pre = 0.0;
    // find the k-st, here the middle item
    let mut k = len / 2;
    let mut i = k / 2;

    loop {

        if nums1_iter >= nums1.len() && nums2_iter >= nums2.len() {
            break;
        }

        // get the next k-th one. 如果len偶数这里取第二个中位数；如果是奇数，这里取到中位数；
        if k == 0 {
            if nums1_iter >= nums1.len() {
                num_half = nums2[nums2_iter] as f64;
            } else if nums2_iter >= nums2.len() {
                num_half = nums1[nums1_iter] as f64;
            } else {
                let num1 = nums1[nums1_iter];
                let num2 = nums2[nums2_iter];
                if num1 < num2 {
                    num_half = num1 as f64;
                } else {
                    num_half = num2 as f64;
                }
            }
            break;
        }

        // for the last 2nd one.当i为0时，如果len是偶数这里找到第一个中位数；如果是奇数，只是取到中位数前一个；
        i = max(1, i);
        let skipped;
        if nums1_iter >= nums1.len() {
            nums2_iter = nums2_iter + i;
            num_half_pre = nums2[min(nums2_iter-1, nums2.len()-1)] as f64;
            skipped = if nums2_iter > nums2.len() {
                i - (nums2_iter - nums2.len())
            } else {
                i
            }
        } else if nums2_iter >= nums2.len() {
            nums1_iter = nums1_iter + i;
            num_half_pre = nums1[min(nums1_iter-1, nums1.len()-1)] as f64;
            skipped = if nums1_iter > nums1.len() {
                i - (nums1_iter - nums1.len())
            } else {
                i
            }
        } else {
            let num1 = nums1[min(nums1_iter+i-1, nums1.len()-1)];
            let num2 = nums2[min(nums2_iter+i-1, nums2.len()-1)];
            if num1 < num2 {
                nums1_iter = nums1_iter + i;
                skipped = if nums1_iter > nums1.len() {
                    i - (nums1_iter - nums1.len())
                } else {
                    i
                };
                num_half_pre = num1 as f64;
            } else {
                nums2_iter = nums2_iter + i;
                skipped = if nums2_iter > nums2.len() {
                    i - (nums2_iter - nums2.len())
                } else {
                    i
                };
                num_half_pre = num2 as f64;
            }
        }

        k = k - skipped;
        i = k / 2;
    }

    if len % 2 == 0 {
        (num_half_pre + num_half)/2.0
    } else {
        num_half
    }
}

// fn find_median_sorted_arrays3(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
//     let mut nums1_iter = 0;
//     let mut nums2_iter = 0;
//     let len = nums1.len() + nums2.len();
//     let mut num_half = 0.0;
//     let mut num_half_pre = 0.0;
//     let mut i = 0;
//     loop {
//         if nums1_iter == nums1.len() && nums2_iter == nums2.len() {
//             break;
//         }
//
//         let num;
//         if nums1_iter == nums1.len() {
//             num = nums2.get(nums2_iter..nums2_iter+1).unwrap().first().unwrap();
//             nums2_iter = nums2_iter + 1;
//         } else if nums2_iter == nums2.len() {
//             num = nums1.get(nums1_iter..nums1_iter+1).unwrap().first().unwrap();
//             nums1_iter = nums1_iter + 1;
//         } else {
//             let num1 = nums1.get(nums1_iter..nums1_iter+1).unwrap().first().unwrap();
//             let num2 = nums2.get(nums2_iter..nums2_iter+1).unwrap().first().unwrap();
//             if num1 < num2 {
//                 num = num1;
//                 nums1_iter = nums1_iter + 1;
//             } else {
//                 num = num2;
//                 nums2_iter = nums2_iter + 1;
//             }
//         }
//
//         if len > 1 && i == len/2-1 {
//             num_half_pre = num.clone() as f64;
//         } else if i == len/2 {
//             num_half = num.clone() as f64;
//             break;
//         }
//
//         i = i + 1;
//     }
//
//     if len % 2 == 0 {
//         (num_half_pre + num_half)/2.0
//     } else {
//         num_half
//     }
// }

//nums1 = [1, 2]
//nums2 = [3, 4]
//
//则中位数是 (2 + 3)/2 = 2.5
// fn find_median_sorted_arrays2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
//     let mut nums: Vec<i32> = [&nums1[..], &nums2[..]].concat();
//     nums.sort();
//
//     let len = nums.len();
//     if len % 2 == 0 {
//         (nums[len/2-1] as f64 + nums[len/2] as f64)/2.0
//     } else {
//         nums[len/2] as f64
//     }
// }

#[test]
fn test_find_median_sorted_arrays() {
    let res = find_median_sorted_arrays(vec![1,2,3], vec![4,5]);
    assert_eq!(res, 3.0);

    let res = find_median_sorted_arrays(vec![1,2], vec![1,2,3]);
    assert_eq!(res, 2.0);

    let res = find_median_sorted_arrays(vec![1,2], vec![3,4]);
    assert_eq!(res, 2.5);

    let res = find_median_sorted_arrays(vec![1,3], vec![2]);
    assert_eq!(res, 2.0);

    let res = find_median_sorted_arrays(vec![], vec![2]);
    assert_eq!(res, 2.0);

    let res = find_median_sorted_arrays(vec![2], vec![]);
    assert_eq!(res, 2.0);

    let res = find_median_sorted_arrays(vec![3], vec![-2, -1]);
    assert_eq!(res, -1.0);
}