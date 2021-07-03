// 给你价格数组 costs 和现金量 coins ，请你计算并返回 Tony 用 coins 现金能够买到的雪糕的 最大数量 。
pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    let mut cost_ordered = costs.to_vec();
    cost_ordered.sort();
    let mut sum = 0;
    let mut count = 0;
    for (i, cost) in cost_ordered.iter().enumerate() {
        sum += *cost;
        if sum <= coins {
            count = i + 1;
        } else {
            break;
        }
    }
    count as i32
}

pub fn max_ice_cream2(costs: Vec<i32>, coins: i32) -> i32 {
    let mut cost_ordered = vec![0;100001];
    for cost in costs.iter() {
        cost_ordered[*cost as usize] += 1;
    }
    let mut sum = 0;
    let mut count = 0;
    'root: for (cost, &num) in cost_ordered.iter().enumerate() {
        for _ in 0..num {
            sum += cost;
            count += 1;
            if sum > coins as usize {
                count = count - 1;
                break 'root;
            }
        }
    }
    count as i32
}

#[test]
fn max_ice_cream_t0() {
    // 输入：costs = [1,3,2,4,1], coins = 7
    // 输出：4
    // 解释：Tony 可以买下标为 0、1、2、4 的雪糕，总价为 1 + 3 + 2 + 1 = 7
    assert_eq!(4, max_ice_cream2(vec![1,3,2,4,1], 7));
}

#[test]
fn max_ice_cream_t1() {
    // 输入：costs = [10,6,8,7,7,8], coins = 5
    // 输出：0
    // 解释：Tony 没有足够的钱买任何一支雪糕。
    assert_eq!(0, max_ice_cream2(vec![10,6,8,7,7,8], 5));
}