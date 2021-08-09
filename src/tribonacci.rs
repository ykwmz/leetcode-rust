// 泰波那契序列 Tn 定义如下： 
//
// T0 = 0, T1 = 1, T2 = 1, 且在 n >= 0 的条件下 Tn+3 = Tn + Tn+1 + Tn+2
//
// 给你整数 n，请返回第 n 个泰波那契数 Tn 的值。
//
pub fn tribonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    match n {
        0 => a,
        1 => b,
        2 => c,
        n => {
            for _ in 3..=n {
                let d = a + b + c;
                a = b;
                b = c;
                c = d;
            }
            c
        }
    }
}

#[test]
fn tribonacci_t1() {
    assert_eq!(1389537, tribonacci(25));
}

#[test]
fn tribonacci_t0() {
    assert_eq!(4, tribonacci(4));
}