use std::collections::HashSet;

// 死亡列表 deadends 的长度范围为 [1, 500]。
// 目标数字 target 不会在 deadends 之中。
// 每个 deadends 和 target 中的字符串的数字会在 10,000 个可能的情况 '0000' 到 '9999' 中产生。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/open-the-lock
pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let deadends_map:HashSet<String> = deadends.into_iter().collect();
    let mut count = -1;
    let mut lock_nums:HashSet<String> = HashSet::new();
    lock_nums.insert("0000".to_string());
    let mut lock_nums_passed:HashSet<String> = HashSet::new();

    loop {
        if lock_nums.is_empty() {
            return -1;
        }
        count += 1;
        let mut lock_nums_next:HashSet<String> = HashSet::new();
        for lock_num in lock_nums.iter() {
            if deadends_map.contains(lock_num) {
                continue;
            } else if target == *lock_num {
                return count;
            } else {
                // make array for next bfs
                for i in 0..lock_num.len() {
                    let c = lock_num.chars().nth(i).unwrap();
                    let c_prev = match c {
                        '0'  => '9',
                        '1'  => '0',
                        '2'  => '1',
                        '3'  => '2',
                        '4'  => '3',
                        '5'  => '4',
                        '6'  => '5',
                        '7'  => '6',
                        '8'  => '7',
                        '9'  => '8',
                        _ => '0'
                    };
                    let c_next = match c {
                        '0'  => '1',
                        '1'  => '2',
                        '2'  => '3',
                        '3'  => '4',
                        '4'  => '5',
                        '5'  => '6',
                        '6'  => '7',
                        '7'  => '8',
                        '8'  => '9',
                        '9'  => '0',
                        _ => '0'
                    };

                    let mut str_prev = lock_num.clone();
                    str_prev.replace_range(i..=i, c_prev.to_string().as_str());
                    if ! lock_nums_passed.contains(&str_prev) {
                        lock_nums_next.insert(str_prev);
                    }

                    let mut str_next = lock_num.clone();
                    str_next.replace_range(i..=i, c_next.to_string().as_str());
                    if ! lock_nums_passed.contains(&str_next) {
                        lock_nums_next.insert(str_next);
                    }
                }
            }
            lock_nums_passed.insert(lock_num.clone());
        }
        lock_nums = lock_nums_next;
    }
}

#[test]
fn open_lock_t0(){
    // 输入：deadends = ["0201","0101","0102","1212","2002"], target = "0202"
    // 输出：6
    // 解释：
    // 可能的移动序列为 "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202"。
    // 注意 "0000" -> "0001" -> "0002" -> "0102" -> "0202" 这样的序列是不能解锁的，
    // 因为当拨动到 "0102" 时这个锁就会被锁定。
    let deadends:Vec<String> = vec!["0201".to_string(),"0101".to_string(),"0102".to_string(),"1212".to_string(),"2002".to_string()];
    assert_eq!(6, open_lock(deadends, "0202".to_string()));
}