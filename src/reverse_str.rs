// 给定一个字符串 s 和一个整数 k，从字符串开头算起，每 2k 个字符反转前 k 个字符。
//
// 如果剩余字符少于 k 个，则将剩余字符全部反转。
// 如果剩余字符小于 2k 但大于或等于 k 个，则反转前 k 个字符，其余字符保持原样。
//
pub fn reverse_str(s: String, k: i32) -> String {
    let mut s_bytes = s.into_bytes();
    let key = k as usize;
    let len = s_bytes.len();
    let mut len_left = s_bytes.len();

    loop {
        match len_left  {
            l if l >= key*2 => {
                s_bytes[len-l..len-l+key].reverse();
                len_left -= key*2;
                continue;
            },
            l if l >= key => {
                s_bytes[len-l..len-l+key].reverse();
                break;
            },
            l => {
                s_bytes[len-l..len].reverse();
                break;
            }
        }
    }

    unsafe {String::from_utf8_unchecked(s_bytes)}
}

#[test]
fn reverse_str_t1() {
    // 输入：s = "abcd", k = 2
    // 输出："bacd"
    assert_eq!("bacd".to_owned(), reverse_str("abcd".to_owned(), 2));
}

#[test]
fn reverse_str_t0() {
    // 输入：s = "abcdefg", k = 2
    // 输出："bacdfeg"
    assert_eq!("bacdfeg".to_owned(), reverse_str("abcdefg".to_owned(), 2));
}