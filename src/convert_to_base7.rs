// 给定一个整数 num，将其转化为 7 进制，并以字符串形式输出。
pub fn convert_to_base7(num: i32) -> String {
    let minus = num < 0;
    let mut n = num.abs();
    let mut str = String::new();
    loop {
        let shang = n / 7;
        let yu = n % 7;
        str.push_str(yu.to_string().as_str());
        n = shang;
        if n <= 0 {
            break;
        }
    }

    if minus { str.push('-') };
    str.chars().rev().collect()
}

#[test]
fn convert_to_base7_t1() {
    // 输入: num = -7
    // 输出: "-10"
    assert_eq!("-10".to_string(), convert_to_base7(-7));
}

#[test]
fn convert_to_base7_t0() {
    // 输入: num = 100
    // 输出: "202"
    assert_eq!("202".to_string(), convert_to_base7(100));
}