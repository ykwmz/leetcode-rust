use std::collections::HashMap;
use std::iter::FromIterator;

// 编写一种方法，对字符串数组进行排序，将所有变位词组合在一起。变位词是指字母相同，但排列不同的字符串。
// 所有输入均为小写字母。
// 不考虑答案输出的顺序。
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // 遍历每个字符串，根据排完序的字符串做hash，最后遍历hash表得出答案；
    // 因为变为词的共同特征就是排序后相等。
    let mut group_ordered:HashMap<String, Vec<String>> = HashMap::new();
    for str in strs.iter() {
        let mut str_ordered :Vec<char>= str.chars().collect();
        str_ordered.sort();
        let key_ordered = String::from_iter(str_ordered);
        if let Some(group) = group_ordered.get_mut(&key_ordered) {
            group.push(str.clone());
        } else {
            group_ordered.insert(key_ordered, vec![str.clone()]);
        }
    }

    let group_final = group_ordered.iter()
        .map(|(_,g)|g.clone()).collect();

    group_final
}

#[test]
fn group_anagrams_t0() {
    // 输入: ["eat", "tea", "tan", "ate", "nat", "bat"],
    // 输出:
    // [
    //   ["ate","eat","tea"],
    //   ["nat","tan"],
    //   ["bat"]
    // ]
    let result = group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]);
    assert_eq!(vec![
        vec!["tan".to_string(), "nat".to_string()],
        vec!["eat".to_string(),"tea".to_string(),"ate".to_string()],
        vec!["bat".to_string()]
    ], result);
}