use std::collections::HashMap;

// 0 <= amount (总金额) <= 5000
// 1 <= coin (硬币面额) <= 5000
// 硬币种类不超过 500 种
// 结果符合 32 位符号整数
pub fn change2(amount: i32, coins: Vec<i32>) -> i32 {
    let mut amount_counts :HashMap<i32, i32> = HashMap::new();
    amount_counts.insert(0, 1);
    // from 0 to sum
    for &coin in coins.iter() {
        let mut amount_counts_cur: HashMap<i32, i32> = amount_counts.clone();
        for (&amount_last, &count_last) in amount_counts.iter() {
            if amount_last > amount {
                continue;
            }
            // case +next*n (每个数字都可能出现n次，并且n最多出现和为amount的个数，否则不需要重复计算下去，*关键退出条件)
            for num in 1..amount/coin+1 {
                let key = amount_last + coin*num;
                if key > amount {
                    break;
                }
                if let Some(c) = amount_counts_cur.get_mut(&key) {
                    *c += count_last;
                } else {
                    amount_counts_cur.insert(key, count_last);
                }
            }
            // case skip
        }
        amount_counts = amount_counts_cur;
    }

    *amount_counts.get(&amount).unwrap_or(&0)
}

pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let mut amount_counts = vec![0; (amount + 1) as usize];
    amount_counts[0] = 1;
    // from 0 to sum
    for &coin in coins.iter() {
        if coin > amount {
            continue;
        }
        let mut amount_counts_cur= amount_counts.clone();
        for (amount_last, &count_last) in amount_counts.iter().enumerate() {
            if amount_last as i32 + coin > amount {
                break;
            }
            // case +next*n (每个数字都可能出现n次，并且n最多出现和为amount的个数，否则不需要重复计算下去，*关键退出条件)
            for num in 1..amount/coin+1 {
                let key = amount_last as i32 + coin*num;
                if key as i32 > amount {
                    break;
                }
                amount_counts_cur[key as usize] += count_last;
            }
            // case skip
        }
        amount_counts = amount_counts_cur;
    }

    *amount_counts.get(amount as usize).unwrap_or(&0)
}

#[test]
fn change_2 () {
    // 500
    // [3,5,7,8,9,10,11]
    assert_eq!(35502874, change(500, vec![3,5,7,8,9,10,11]));
}

#[test]
fn change_1 () {
    // 0,[7]
    assert_eq!(1, change(0, vec![7]));
}

#[test]
fn change_0 () {
    // 输入: amount = 5, coins = [1, 2, 5]
    // 输出: 4
    // 解释: 有四种方式可以凑成总金额:
    // 5=5
    // 5=2+2+1
    // 5=2+1+1+1
    // 5=1+1+1+1+1
    assert_eq!(4, change(5, vec![1,2,5]));
}