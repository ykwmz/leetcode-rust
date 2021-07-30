// 给你一个字符串 columnTitle ，表示 Excel 表格中的列名称。返回该列名称对应的列序号。
//
//  
//
// 例如，
//
//     A -> 1
//     B -> 2
//     C -> 3
//     ...
//     Z -> 26
//     AA -> 27
//     AB -> 28
//     ...
// 1 <= columnTitle.length <= 7
// columnTitle 仅由大写英文组成
// columnTitle 在范围 ["A", "FXSHRXW"] 内
//
pub fn title_to_number(column_title: String) -> i32 {
    // let titles = ['Z','A','B','C','D','E','F','G','H','I','J','K','L'
    // ,'M','N','O','P','Q','R','S','T','U','V','W','X','Y'];
    let size = column_title.len();
    let mut number = 0;
    for (i,c) in column_title.chars().enumerate() {
        let differ = size - i - 1;
        number += (c as u32 - 'A' as u32 + 1) * 26_u32.pow(differ as u32);
    }

    number as i32
}

#[test]
fn title_to_number_t3() {
    // 输入: columnTitle = "FXSHRXW"
    // 输出: 2147483647
    assert_eq!(2147483647, title_to_number("FXSHRXW".to_string()));
}

#[test]
fn title_to_number_t2() {
    // 输入: columnTitle = "ZY"
    // 输出: 701
    assert_eq!(701, title_to_number("ZY".to_string()));
}

#[test]
fn title_to_number_t1() {
    // 输入: columnTitle = "AB"
    // 输出: 28
    assert_eq!(28, title_to_number("AB".to_string()));
}

#[test]
fn title_to_number_t0() {
    // 输入: columnTitle = "A"
    // 输出: 1
    assert_eq!(1, title_to_number("A".to_string()));
}

