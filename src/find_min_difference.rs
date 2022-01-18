// 给定一个 24 小时制（小时:分钟 "HH:MM"）的时间列表，找出列表中任意两个时间的最小时间差并以分钟数表示。
// 2 <= timePoints.length <= 2 * 10^4
// timePoints[i] 格式为 "HH:MM"

use std::cmp::min;

pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let mut points = vec![0i16; 1441];
    for item in time_points {
        let part:Vec<&str> = item.split(':').collect();
        let left:i32  = part[0].parse().unwrap();
        let right:i32 = part[1].parse().unwrap();
        let minutes = (left*60+right) as usize;
        points[minutes] = points[minutes] + 1;

        if points[minutes] > 1 {
            return 0;
        }
    };

    let mut minimal = i16::MAX;
    for i in 0..1441i16 {
        if points[i as usize] <= 0 {
            continue;
        }
        for j in i+1..1441i16 {
            if points[j as usize] <= 0 {
                continue;
            }
            minimal = min(minimal, min(j - i, i + 1440 - j));
        }
    }

    minimal as i32
}

#[test]
pub fn find_min_difference_t3()  {
    // 输入：timePoints = ["12:12","12:13","00:12","00:13"]
    // 输出：1
    assert_eq!(1, find_min_difference(vec!["12:12".to_string(),"12:13".to_string(),
                                           "00:12".to_string(),"00:13".to_string()]))
}

#[test]
pub fn find_min_difference_t2()  {
    // 输入：timePoints = ["00:00","23:59"]
    // 输出：1
    assert_eq!(1, find_min_difference(vec!["00:00".to_string(),"23:59".to_string()]))
}

#[test]
pub fn find_min_difference_t1()  {
    // 输入：timePoints = ["00:00","23:59","00:00"]
    // 输出：0
    assert_eq!(0, find_min_difference(vec!["00:00".to_string(),"23:59".to_string(),"00:00".to_string()]))
}

#[test]
pub fn find_min_difference_t0() {
    // 输入：timePoints = ["23:59","00:00"]
    // 输出：1
    assert_eq!(1, find_min_difference(vec!["23:59".to_string(),"00:00".to_string()]))
}