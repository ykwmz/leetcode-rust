use std::collections::HashMap;
use std::cmp::min;

//1 <= n <= 100
// 0 <= minProfit <= 100
// 1 <= group.length <= 100
// 1 <= group[i] <= 100
// profit.length == group.length
// 0 <= profit[i] <= 100
//
// NO.879
pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
    // let mut n_min_profit = (n, min_profit);
    // let mut count = if min_profit == 0 {1} else {0};
    let mut n_min_profits:HashMap<(i32,i32),i32> = HashMap::new();
    n_min_profits.insert((0,0),1);
    let group_profits = group.into_iter().zip(profit.into_iter());
    for (group_item, profit_item) in group_profits {
        let mut n_min_profits_next = n_min_profits.clone();
        for ((n_left,min_profit_left), count) in n_min_profits.iter() {
            let key = (*n_left+group_item, min(*min_profit_left+profit_item,min_profit) );
            if *n_left+group_item > n {
                continue;
            }
            // case +
            if let Some(count_cur) = n_min_profits_next.get_mut(&(key)) {
                *count_cur = (*count_cur + *count) % (1e9 as i32+7);
                // println!("{}",*count_cur);
            } else {
                n_min_profits_next.insert(key, *count % (1e9 as i32+7));
            }
            // case skip
            // n_min_profits_next.insert((*n_left, min(*min_profit_left,min_profit)), *count);
        }
        n_min_profits = n_min_profits_next;
    }
    // println!("{:?}", n_min_profits);
    let result:HashMap<(i32,i32),i32> = n_min_profits.into_iter().filter(|((x,y),_)|*x <= n && *y >= min_profit).collect();
    let values = result.values().map(|v|*v as i64).collect::<Vec<i64>>();
    let sum:i64 = values.into_iter().sum();
    (sum % (1e9 as i64+7)) as i32
}

#[test]
fn profitable_schemes_5() {
    //100
    // 100
    let group =  vec![2,5,36,2,5,5,14,1,12,1,14,15,1,1,27,13,6,59,6,1,7,1,2,7,6,1,6,1,3,1,2,11,3,39,21,20,1,27,26,22,11,17,3,2,4,5,6,18,4,14,1,1,1,3,12,9,7,3,16,5,1,19,4,8,6,3,2,7,3,5,12,6,15,2,11,12,12,21,5,1,13,2,29,38,10,17,1,14,1,62,7,1,14,6,4,16,6,4,32,48];
    let profit = vec![21,4,9,12,5,8,8,5,14,18,43,24,3,0,20,9,0,24,4,0,0,7,3,13,6,5,19,6,3,14,9,5,5,6,4,7,20,2,13,0,1,19,4,0,11,9,6,15,15,7,1,25,17,4,4,3,43,46,82,15,12,4,1,8,24,3,15,3,6,3,0,8,10,8,10,1,21,13,10,28,11,27,17,1,13,10,11,4,36,26,4,2,2,2,10,0,11,5,22,6];
    assert_eq!(692206787, profitable_schemes(100, 100, group, profit));
}

#[test]
fn profitable_schemes_4() {
    //1
    // 1
    let group =  vec![2,2,2,2,2,2,1,2,1,1,2,1,2,2,2,1,2,1,1,2,1,2,1,2,2,2,2,1,1,2,2,2,1,1,2,1,2,2,2,1,2,2,2,2,1,2,2,1,2,2,1,1,1,1,1,1,2,2,2,2,1,1,1,2,1,1,1,2,1,1,1,2,1,1,1,2,2,1,1,2,2,2,1,1,2,2,1,1,2,2,1,2,2,1,1,2,2,2,2,2];
    let profit = vec![2,1,2,2,2,1,0,1,2,0,0,2,2,1,1,1,2,0,1,1,2,0,2,2,1,0,1,0,1,2,2,1,1,2,0,2,1,1,1,1,1,2,0,1,0,2,2,2,2,2,0,1,1,2,1,0,1,0,0,2,1,0,2,0,2,1,1,1,0,1,0,1,2,2,0,1,1,2,2,0,1,0,0,1,1,2,2,2,2,1,0,0,1,2,1,1,1,1,0,1];
    assert_eq!(31, profitable_schemes(1, 1, group, profit));
}

#[test]
fn profitable_schemes_3() {
    //100
    // 10
    let group = vec![66,24,53,49,86,37,4,70,99,68,14,91,70,71,70,98,48,26,13,86,4,82,1,7,51,37,27,87,2,65,93,66,99,28,17,93,83,91,45,3,59,87,92,62,77,21,9,37,11,4,69,46,70,47,28,40,74,47,12,3,85,16,91,100,39,24,52,50,40,23,64,22,2,15,18,62,26,76,3,60,64,34,45,40,49,11,5,8,40,71,12,60,3,51,31,5,42,52,15,36];
    let profit = vec![8,4,8,8,9,3,1,6,7,10,1,10,4,9,7,11,5,1,7,4,11,1,5,9,9,5,1,10,0,10,4,1,1,1,6,9,3,6,2,5,4,7,8,5,2,3,0,6,4,5,9,9,10,7,1,8,9,6,0,2,9,2,2,8,6,10,3,4,6,1,10,7,5,4,8,1,8,5,5,4,1,1,10,0,8,0,1,11,5,4,7,9,1,11,1,0,1,6,8,3];
    assert_eq!(188883405, profitable_schemes(100, 10, group, profit));
}

#[test]
fn profitable_schemes_2() {
    //输入：n = 64, minProfit = 0, group = [80, 40], profit = [88, 88]
    // 输出：2
    assert_eq!(2, profitable_schemes(64, 0, vec![80,40], vec![88,88]));
}

#[test]
fn profitable_schemes_1() {
    //输入：n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
    // 输出：7
    // 解释：至少产生 5 的利润，只要完成其中一种工作就行，所以该集团可以完成任何工作。
    // 有 7 种可能的计划：(0)，(1)，(2)，(0,1)，(0,2)，(1,2)，以及 (0,1,2) 。
    assert_eq!(7, profitable_schemes(10, 5, vec![2,3,5], vec![6,7,8]));
}

#[test]
fn profitable_schemes_0() {
    //输入：n = 5, minProfit = 3, group = [2,2], profit = [2,3]
    // 输出：2
    // 解释：至少产生 3 的利润，该集团可以完成工作 0 和工作 1 ，或仅完成工作 1 。
    // 总的来说，有两种计划。
    assert_eq!(2, profitable_schemes(5, 3, vec![2,2], vec![2,3]));
}