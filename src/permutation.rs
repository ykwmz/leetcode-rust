// 输入一个字符串，打印出该字符串中字符的所有排列。
// 你可以以任意顺序返回这个字符串数组，但里面不能有重复元素。
// 1 <= s 的长度 <= 8
pub fn permutation(s: String) -> Vec<String> {
    let str_vec = Vec::from(s);
    let mut target_vec:Vec<String> = vec![(*str_vec.get(0).unwrap() as char).to_string()];

    for c in str_vec[1..].iter() {
        let mut target_cur = Vec::new();
        for str in target_vec.iter() {
            for i in 0..str.len()+1 {
                let mut str_new = str.clone();
                str_new.insert(i, *c as char);
                target_cur.push(str_new);
            }
        }
        target_vec = target_cur;
    }

    target_vec.sort_unstable();
    target_vec.dedup();
    target_vec
}

#[test]
fn permutation_t0() {
    // 输入：s = "abc"
    // 输出：["abc","acb","bac","bca","cab","cba"]
    assert_eq!(vec!["abc","acb","bac","bca","cab","cba"], permutation("abc".to_string()))
}