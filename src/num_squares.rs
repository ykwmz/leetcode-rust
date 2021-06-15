use std::cmp::min;

// 1 <= n <= 10^4
pub fn num_squares(n: i32) -> i32 {
    let mut squares = vec![10000;(n+1) as usize];
    for i in 0..2 {
        squares[i as usize] = i;
    }
    // from 0 to sum
    for num in 1..n+1 {
        let max = (num as f64).sqrt().ceil() as i32 + 1;
        for sqrt in 0..max {
            let square = sqrt * sqrt;
            if num < square {
                break;
            }
            squares[num as usize] = min(squares[num as usize], squares[(num - square) as usize] + 1);
        }
    }

    *squares.get(n as usize).unwrap_or(&0)
}

#[test]
fn num_squares_5() {
    // 输入：n = 2
    // 输出：2
    assert_eq!(2, num_squares(2));
}

#[test]
fn num_squares_4() {
    // 输入：n = 9998
    // 输出：3
    assert_eq!(3, num_squares(9998));
}

#[test]
fn num_squares_3() {
    // 输入：n = 4
    // 输出：1
    assert_eq!(1, num_squares(4));
}

#[test]
fn num_squares_2() {
    // 输入：n = 1
    // 输出：1
    assert_eq!(1, num_squares(1));
}

#[test]
fn num_squares_1() {
    // 输入：n = 13
    // 输出：2
    // 解释：13 = 4 + 9
    assert_eq!(2, num_squares(13));
}

#[test]
fn num_squares_0() {
    // 输入：n = 12
    // 输出：3
    // 解释：12 = 4 + 4 + 4
    assert_eq!(3, num_squares(12));
}