// 给定总玩家数 n，以及按 [玩家编号,对应可传递玩家编号] 关系组成的二维数组 relation。返回信息从小 A (编号 0 ) 经过 k 轮传递到编号为 n-1 的小伙伴处的方案数；若不能到达，返回 0。
// 限制：
// 2 <= n <= 10
// 1 <= k <= 5
// 1 <= relation.length <= 90, 且 relation[i].length == 2
// 0 <= relation[i][0],relation[i][1] < n 且 relation[i][0] != relation[i][1]
pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut relations:Vec<Vec<i32>> = vec![Vec::new();10];
    for rel in relation.iter() {
        relations[rel[0] as usize].push(rel[1]);
    }

    let mut ways = vec![&0];
    let mut i = 0;
    while i < k {
        ways = ways.iter().flat_map(|&&w|{
            relations.get(w as usize).unwrap()
        }).collect();

        i += 1;
    }

    ways.iter().filter(|&&w| *w == n - 1).count() as i32
}

#[test]
fn num_ways_t1() {
    // 输入：n = 3, relation = [[0,2],[2,1]], k = 2
    // 输出：0
    // 解释：信息不能从小 A 处经过 2 轮传递到编号 2
    assert_eq!(0, num_ways(3, vec![vec![0,2],vec![2,1]], 2));
}

#[test]
fn num_ways_t0() {
    // 输入：n = 5, relation = [[0,2],[2,1],[3,4],[2,3],[1,4],[2,0],[0,4]], k = 3
    // 输出：3
    // 解释：信息从小 A 编号 0 处开始，经 3 轮传递，到达编号 4。共有 3 种方案，分别是 0->2->0->4， 0->2->1->4， 0->2->3->4。
    assert_eq!(3, num_ways(5, vec![vec![0,2],vec![2,1],vec![3,4],vec![2,3],vec![1,4],vec![2,0],vec![0,4]], 3));
}