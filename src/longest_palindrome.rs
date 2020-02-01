
fn longest_palindrome(s: String) -> String {
    let mut left = 0;
    let mut right = 0;
    let fn_expand_compare = |left:usize, right:usize, s:&String|->(usize,usize){
        let mut sub_left = left;
        let mut sub_right = right;
        while sub_left>=0 && sub_right<s.len() && s.get(sub_left..sub_left+1).unwrap() == s.get(sub_right..sub_right+1).unwrap() {
            sub_right = sub_right + 1;
            if sub_left == 0 {
                return (sub_left, sub_right);
            } else {
                sub_left = sub_left - 1;
            }
        }
        (sub_left+1, sub_right)
    };
    for i in 0..s.len() {
        // check with 奇数
        let (sub_left, sub_right) = fn_expand_compare(i, i, &s);
        if sub_right - sub_left > right - left {
            left = sub_left;
            right = sub_right;
        }
        // check with 偶数
        let (sub_left, sub_right) = fn_expand_compare(i, i+1, &s);
        if sub_right - sub_left > right - left {
            left = sub_left;
            right = sub_right;
        }
    }

    return if left >= right {
        String::new()
    } else {
        s.get(left..right).unwrap().to_string()
    }
}

fn longest_palindrome2(s: String) -> String {
    let mut sub_len = s.len();
    loop {
        if sub_len == 0 {
            break;
        } else if sub_len == 1 {
            return s.get(0..sub_len).unwrap().to_string();
        }

        for i in 0..(s.len()-sub_len+1) {
            // check palindrome half
            let mut sub_left = Vec::from(s.get(i..i+sub_len/2).unwrap());
            let sub_right = Vec::from(if sub_len%2==0 {
                s.get(i+sub_len/2..i+sub_len).unwrap()
            } else {
                s.get(i+sub_len/2+1..i+sub_len).unwrap()
            });
            sub_left.reverse();

            // if found return to res directly
            if sub_left.eq(&sub_right) {
                return s.get(i..i+sub_len).unwrap().to_string();
            }
        }

        sub_len = sub_len - 1;
    }

    return String::new();
}

//输入: "babad"
//输出: "bab"
//注意: "aba" 也是一个有效答案。
#[test]
fn test_longest_palindrome() {
    let res = longest_palindrome(String::from("babad"));
    assert_eq!(res, "bab");

    let res = longest_palindrome(String::from("cbbd"));
    assert_eq!(res, "bb");

    let res = longest_palindrome(String::from("a"));
    assert_eq!(res, "a");

    let res = longest_palindrome(String::from("ac"));
    assert_eq!(res, "a");
}