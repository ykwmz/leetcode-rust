use std::cmp::max;

fn length_of_longest_substring(s: String) -> i32 {
    let mut res_len = 0;
    let mut start = 0;
    for (cur, c) in s.chars().enumerate() {
        if let Some(recur_i) = s.get(start..cur).unwrap().find(c) {
            res_len = max(res_len, cur - start);
            // only search start at the found same item's index + 1 ~, because there have already have recurred items following.
            start = start + recur_i + 1;
        }
    }
    res_len = max(res_len, s.len() - start);
    return res_len as i32;
}

//输入: "abcabcbb"
//输出: 3
//解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
// fn length_of_longest_substring2(s: String) -> i32 {
//     let mut res_len = 0;
//     for i in 0..s.len() {
//         let mut set = HashSet::new();
//         set.insert(s.get(i..i+1).unwrap());
//         for j in i+1..s.len() {
//             if let false = set.insert(s.get(j..j+1).unwrap()) {
//                 break;
//             }
//         }
//         res_len = max(res_len, set.len());
//     }
//
//     return res_len as i32;
// }

#[test]
fn test_length_of_longest_substring() {
    let res = length_of_longest_substring(String::from("abcabcbb"));
    assert_eq!(res, 3);

    let res = length_of_longest_substring(String::from("bbbbb"));
    assert_eq!(res, 1);

    let res = length_of_longest_substring(String::from("a"));
    assert_eq!(res, 1);

    let res = length_of_longest_substring(String::from("au"));
    assert_eq!(res, 2);
}