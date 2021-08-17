// 给你一个字符串 s 表示一个学生的出勤记录，其中的每个字符用来标记当天的出勤情况（缺勤、迟到、到场）。记录中只含下面三种字符：
//
// 'A'：Absent，缺勤
// 'L'：Late，迟到
// 'P'：Present，到场
// 如果学生能够 同时 满足下面两个条件，则可以获得出勤奖励：
//
// 按 总出勤 计，学生缺勤（'A'）严格 少于两天。
// 学生 不会 存在 连续 3 天或 3 天以上的迟到（'L'）记录。
// 如果学生可以获得出勤奖励，返回 true ；否则，返回 false 。
//
// 1 <= s.length <= 1000
// s[i] 为 'A'、'L' 或 'P'
//
pub fn check_record(s: String) -> bool {
    let mut absent = 0;
    let mut present_continues = 0;
    for char in s.chars() {
        match char {
            'A' => {
                absent += 1;
                present_continues = 0;
            },
            'L' => present_continues += 1,
            _ => present_continues = 0
        }

        // if not meet, return directly
        if absent >= 2 || present_continues >= 3 {
            return false;
        }
    }

    true
}

#[test]
fn check_record_t2() {
    assert_eq!(false, check_record("LLLALL".to_owned()))
}

#[test]
fn check_record_t1() {
    // 输入：s = "PPALLL"
    // 输出：false
    // 解释：学生最后三天连续迟到，所以不满足出勤奖励的条件。
    assert_eq!(false, check_record("PPALLL".to_owned()))
}

#[test]
fn check_record_t0() {
    // 输入：s = "PPALLP"
    // 输出：true
    // 解释：学生缺勤次数少于 2 次，且不存在 3 天或以上的连续迟到记录。
    assert_eq!(true, check_record("PPALLP".to_owned()))
}