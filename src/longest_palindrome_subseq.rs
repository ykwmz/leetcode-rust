// 给你一个字符串 s ，找出其中最长的回文子序列，并返回该序列的长度。

// 解题思路
// 子序列定义为：不改变剩余字符顺序的情况下，删除某些字符或者不删除任何字符形成的一个序列。
// 我们说这个问题对 dp 数组的定义是：在子串 s[i..j] 中，最长回文子序列的长度为 dp[i][j]。一定要记住这个定义才能理解算法。
// 为啥这个问题要这样定义二维的 dp 数组呢？我们前文多次提到，找状态转移需要归纳思维，说白了就是如何从已知的结果推出未知的部分，这样定义容易归纳，容易发现状态转移关系。
// 具体来说，如果我们想求 dp[i][j]，假设你知道了子问题 dp[i+1][j-1] 的结果（s[i+1..j-1] 中最长回文子序列的长度），你是否能想办法算出 dp[i][j] 的值（s[i..j] 中，最长回文子序列的长度）呢？

// 如果它俩相等，那么它俩加上 s[i+1..j-1] 中的最长回文子序列就是 s[i..j] 的最长回文子序列：
// 如果它俩不相等，说明它俩不可能同时出现在 s[i..j] 的最长回文子序列中，那么把它俩分别加入 s[i+1..j-1] 中，看看哪个子串产生的回文子序列更长即可：
// 

pub fn longest_palindrome_subseq(s: String) -> i32 {
    let len = s.len();
    let mut map = vec![vec![0;len];len];
    for i in 0..len {
        map[i][i] = 1;
    }

    for i in (0..len).rev() {
        for j in i+1..len {
            map[i][j] = if s.as_bytes()[i] == s.as_bytes()[j] {
                    map[i+1][j-1] + 2
                } else {
                    if map[i][j-1] > map[i+1][j] {
                        map[i][j-1]
                    } else {
                        map[i+1][j]
                    }
                }
        }
    }

    map[0][len-1]
}

#[test]
fn longest_palindrome_subseq_t2() {
    assert_eq!(159, longest_palindrome_subseq("euazbipzncptldueeuechubrcourfpftcebikrxhybkymimgvldiwqvkszfycvqyvtiwfckexmowcxztkfyzqovbtmzpxojfofbvwnncajvrvdbvjhcrameamcfmcoxryjukhpljwszknhiypvyskmsujkuggpztltpgoczafmfelahqwjbhxtjmebnymdyxoeodqmvkxittxjnlltmoobsgzdfhismogqfpfhvqnxeuosjqqalvwhsidgiavcatjjgeztrjuoixxxoznklcxolgpuktirmduxdywwlbikaqkqajzbsjvdgjcnbtfksqhquiwnwflkldgdrqrnwmshdpykicozfowmumzeuznolmgjlltypyufpzjpuvucmesnnrwppheizkapovoloneaxpfinaontwtdqsdvzmqlgkdxlbeguackbdkftzbnynmcejtwudocemcfnuzbttcoew".to_string()));
}

#[test]
fn longest_palindrome_subseq_t1() {
    // 输入：s = "cbbd"
    // 输出：2
    // 解释：一个可能的最长回文子序列为 "bb" 。
    assert_eq!(2, longest_palindrome_subseq("cbbd".to_string()));
}

#[test]
fn longest_palindrome_subseq_t0() {
    // 输入：s = "bbbab"
    // 输出：4
    // 解释：一个可能的最长回文子序列为 "bbbb" 。
    assert_eq!(4, longest_palindrome_subseq("bbbab".to_string()));
}