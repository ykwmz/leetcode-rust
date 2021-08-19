// 编写一个函数，以字符串作为输入，反转该字符串中的元音字母。
// 元音字母不包含字母 "y" 。
//
pub fn reverse_vowels(mut s: String) -> String {
    let s_bytes = unsafe { s.as_bytes_mut() };
    let mut l = 0;
    let mut r = s_bytes.len() - 1;
    let is_original = |v:char| {
        match v.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false
        }
    };
    while let Some(&l_v) = s_bytes.get(l) {
        if l >= r {
            break;
        }
        if is_original(char::from(l_v)) {
            while let Some(&r_v) = s_bytes.get(r) {
                if l >= r {
                    break;
                }
                if is_original(char::from(r_v)) {
                    s_bytes.swap(l,r);
                    r -= 1;
                    break;
                }
                r -= 1;
            }
        }
        l += 1;
    }

    unsafe { String::from_utf8_unchecked(s_bytes.to_vec()) }
}

#[test]
fn reverse_vowels_t1() {
    // 输入："leetcode"
    // 输出："leotcede"
    assert_eq!("leotcede".to_owned(), reverse_vowels("leetcode".to_owned()));
}

#[test]
fn reverse_vowels_t0() {
    // 输入："hello"
    // 输出："holle"
    assert_eq!("holle".to_owned(), reverse_vowels("hello".to_owned()));
}