
/*
1 <= candidates.length <= 30
1 <= candidates[i] <= 200
candidate 中的每个元素都是独一无二的。
1 <= target <= 500
*/
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates_ordered:Vec<i32> = candidates.to_vec();
    let mut target_array:Vec<Vec<i32>> = Vec::new();

    pub fn sum(index: usize, candidates: &Vec<i32>, target_sub_array: Vec<i32>, target_sub_sum: i32, target_array: &mut Vec<Vec<i32>>, target_sum: i32 ) {
        if index >= candidates.len() {
            return;
        }
        let cur_candidate = candidates[index];
        let target_sub_sum = cur_candidate + target_sub_sum;
        match target_sub_sum {
            d if d< target_sum => {
                // to self and next all
                for i in 0..candidates.len()-index {
                    sum(index+i, candidates, [target_sub_array.to_vec(), [cur_candidate].to_vec()].concat(), target_sub_sum, target_array, target_sum);
                }
            },
            d if d> target_sum => return,
            d if d== target_sum => {
                target_array.push([target_sub_array.to_vec(), [cur_candidate].to_vec()].concat());
                return;
            }
            _ => return
        }
        return;
    }

    candidates_ordered.sort();
    let _ = match candidates_ordered.iter().rposition(|&v|v<=target) {
        Some(at) => candidates_ordered.split_off(at+1),
        _ => [].to_vec()
    };

    // found by DFS
    for (i,v) in candidates_ordered.iter().enumerate() {
        match v {
            &v if v < target => sum(i, &candidates_ordered, Vec::new(), 0, &mut target_array, target),
            &v if v == target => target_array.insert(0, vec![v]),
            _ => break
        }
    }

    target_array
}

#[cfg(test)]
mod tests {
    use crate::combination_sum::combination_sum;

    #[test]
    fn test_combination_sum4() {
        let candidates = vec![2];
        let target = 1;
        let res = combination_sum(candidates, target);

        assert_eq!(res.len(), 0);
    }
    #[test]
    fn test_combination_sum3() {
        let candidates = vec![3, 5, 8];
        let target = 11;
        let res = combination_sum(candidates, target);

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], [3,3,5]);
        assert_eq!(res[1], [3,8]);
    }

    #[test]
    fn test_combination_sum0() {
        let candidates = vec![7];
        let target = 7;
        let res = combination_sum(candidates, target);

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], [7]);
    }

    #[test]
    fn test_combination_sum1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let res = combination_sum(candidates, target);

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], [7]);
        assert_eq!(res[1], [2,2,3]);
    }
    #[test]
    fn test_combination_sum2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let res = combination_sum(candidates, target);

        assert_eq!(res.len(), 3);
        assert_eq!(res[0], [2,2,2,2]);
        assert_eq!(res[1], [2,3,3]);
        assert_eq!(res[2], [3,5]);
    }
}
