// 创建一个基于时间的键值存储类 TimeMap，它支持下面两个操作：
//
// 1. set(string key, string value, int timestamp)
//
// 存储键 key、值 value，以及给定的时间戳 timestamp。
// 2. get(string key, int timestamp)
//
// 返回先前调用 set(key, value, timestamp_prev) 所存储的值，其中 timestamp_prev <= timestamp。
// 如果有多个这样的值，则返回对应最大的  timestamp_prev 的那个值。
// 如果没有值，则返回空字符串（""）。

// 所有的键/值字符串都是小写的。
// 所有的键/值字符串长度都在 [1, 100] 范围内。
// 所有 TimeMap.set 操作中的时间戳 timestamps 都是严格递增的。
// 1 <= timestamp <= 10^7
// TimeMap.set 和 TimeMap.get 函数在每个测试用例中将（组合）调用总计 120000 次。

use std::collections::HashMap;

struct TimeMap {
    time_map: HashMap<String, Vec<(i32, String)>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    /** Initialize your data structure here. */
    fn new() -> Self {
        TimeMap {
            time_map: HashMap::new()
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if ! self.time_map.contains_key(&key) {
            self.time_map.insert(key.clone(), Vec::new());
        }
        self.time_map.get_mut(&key).unwrap().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(v) = self.time_map.get(&key) {
            // 严格递增, 认为默认是按照timestamp排好序的
            match v.binary_search_by(|(vi,_)|vi.cmp(&timestamp)) {
                Ok(vi) => return v.get(vi).unwrap().1.clone(),
                Err(vi) => {
                    if vi > 0 {
                        return v.get(vi-1).unwrap().1.clone();
                    }
                }
            }
        }
        "".to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
#[test]
fn test_t0() {
    // 输入：inputs = ["TimeMap","set","set","get","get","get","get","get"],
    //       inputs = [[],["love","high",10],["love","low",20],["love",5],["love",10],["love",15],["love",20],["love",25]]
    // 输出：[null,null,null,"","high","high","low","low"]
    let mut obj = TimeMap::new();
    obj.set("love".to_string(), "high".to_string(), 10);
    obj.set("love".to_string(), "low".to_string(), 20);
    assert_eq!("".to_string(), obj.get("love".to_string(), 5));
    assert_eq!("high".to_string(), obj.get("love".to_string(), 10));
    assert_eq!("high".to_string(), obj.get("love".to_string(), 15));
    assert_eq!("low".to_string(), obj.get("love".to_string(), 20));
    assert_eq!("low".to_string(), obj.get("love".to_string(), 25));
}