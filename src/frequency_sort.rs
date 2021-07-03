use std::collections::HashMap;
use std::iter::FromIterator;

// 给定一个字符串，请将字符串里的字符按照出现的频率降序排列。
pub fn frequency_sort(s: String) -> String {
    let mut s_ordered:HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *s_ordered.entry(c).or_insert(0) += 1;
    }
    let mut v_ordered:Vec<(char, i32)> = s_ordered.into_iter().collect();
    v_ordered.sort_by(|&a,&b|b.1.cmp(&a.1));

    String::from_iter(v_ordered.iter().flat_map(|(c, n)|vec![c;*n as usize]))
}

#[test]
fn frequency_sort_t2() {
    assert!(matches!(frequency_sort("Aabb".to_string()).as_str(), "bbAa" | "bbaA"));
}

#[test]
fn frequency_sort_t1() {
    assert!(matches!(frequency_sort("tree".to_string()).as_str() ,"eert" | "eetr"));
}

#[test]
fn frequency_sort_t0() {
    assert!(matches!(frequency_sort("cccaaa".to_string()).as_str() ,"cccaaa" | "aaaccc"));
}