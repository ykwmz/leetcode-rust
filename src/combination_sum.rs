
/*
1 <= candidates.length <= 30
1 <= candidates[i] <= 200
candidate 中的每个元素都是独一无二的。
1 <= target <= 500
*/
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidatesOrdered:Vec<i32> = candidates.to_vec();
    let mut targetArray:Vec<Vec<i32>> = Vec::new();

    pub fn sum(index: usize, candidates: &Vec<i32>, targetSubArray: Vec<i32>, targetSubSum: i32, targetArray: &mut Vec<Vec<i32>>, targetSum: i32 ) {
        if index >= candidates.len() {
            return;
        }
        let curCandidate = candidates[index];
        let targetSubSum = curCandidate + targetSubSum;
        match targetSubSum {
            d if d<targetSum => {
                // to self and next all
                for i in 0..candidates.len()-index {
                    sum(index+i, candidates, [targetSubArray.to_vec(), [curCandidate].to_vec()].concat(), targetSubSum, targetArray, targetSum);
                }
            },
            d if d>targetSum => return,
            d if d==targetSum => {
                targetArray.push([targetSubArray.to_vec(), [curCandidate].to_vec()].concat());
                return;
            }
            _ => return
        }
        return;
    }

    candidatesOrdered.sort();
    let _ = match candidatesOrdered.iter().rposition(|&v|v<=target) {
        Some(at) => candidatesOrdered.split_off(at+1),
        _ => [].to_vec()
    };

    // found by DFS
    for (i,v) in candidatesOrdered.iter().enumerate() {
        match v {
            &v if v < target => sum(i, &candidatesOrdered, Vec::new(), 0, &mut targetArray, target),
            &v if v == target => targetArray.insert(0, vec![v]),
            _ => break
        }
    }

    targetArray
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
