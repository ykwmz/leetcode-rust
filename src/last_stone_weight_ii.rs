use std::collections::HashMap;

// 1049
// 1 <= stones.length <= 30
// 1 <= stones[i] <= 100
// 巧妙的转化为类似于find_target_sum_ways的dp题目，
// 即在每个数前放置+-号（涵盖所有石头组合，诸如 A-(B-C) => A-B+C 或 A-(C-B) => A+B-C），然后逐层累加，sum存为key，至最后一层取最小的sum便是解。
pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    // special cases
    // let mut weight_left = 100;
    // last_stone_weight_ii_inner(stones, &mut weight_left);
    //
    // weight_left
    let mut counts_last:HashMap<i32,i32> = HashMap::new();
    counts_last.insert(0,1);
    for (_, stone) in stones.iter().enumerate() {
        let mut counts_cur:HashMap<i32, i32>= HashMap::new();
        for (sum, count) in counts_last.iter() {
            // case +
            if let Some(count_cur) = counts_cur.get_mut(&(sum+stone)) {
                *count_cur += count;
            } else {
                counts_cur.insert(sum+stone, *count);
            }
            // case -
            if let Some(count_cur) = counts_cur.get_mut(&(sum-stone)) {
                *count_cur += count;
            } else {
                counts_cur.insert(sum-stone, *count);
            }
        }
        counts_last = counts_cur;
    }

    *counts_last.keys().into_iter().filter(|&&k|k >= 0).min().unwrap_or(&0)
    // *counts_last.keys().min().unwrap_or(&0)
}

// fn last_stone_weight_ii_inner(stones: Vec<i32>, weight_left: &mut i32) {
//     // special cases
//     if stones.len() == 1 {
//         *weight_left = min(stones[0], *weight_left);
//     } else if stones.len() == 2 {
//         *weight_left = min((stones[0] - stones[1]).abs(), *weight_left);
//     } else {
//         // let mut stones_cur = stones[1..].to_vec();
//         let stone_first = stones[0];
//         for (j,stone_next) in (&stones[1..]).iter().enumerate() {
//             let mut stones_next = Vec::from(&stones[1..]);
//             if let Some(stone) = stones_next.get_mut(j) {
//                 *stone = (stone_first - *stone_next).abs();
//             }
//             last_stone_weight_ii_inner(stones_next.to_vec(), weight_left);
//         }
//     }
// }

#[test]
fn last_stone_weight_ii_3() {
    let stones = vec![1,1,2,3,5,8,13,21,34,55,89,14,23,37,61,98];
    assert_eq!(1, last_stone_weight_ii(stones));
}
#[test]
fn last_stone_weight_ii_2() {
    let stones = vec![2,7,4,1,8,1];
    assert_eq!(1, last_stone_weight_ii(stones));
}
#[test]
fn last_stone_weight_ii_1() {
    let stones = vec![31,26,33,21,40];
    assert_eq!(5, last_stone_weight_ii(stones));
}
#[test]
fn last_stone_weight_ii_0() {
    let stones = vec![1,2];
    assert_eq!(1, last_stone_weight_ii(stones));
}