use std::str::FromStr;

fn reverse(x: i32) -> i32 {
    // memo negative
    let is_negative = x < 0;
    // to string
    let mut x_str = x.to_string();
    x_str = x_str.trim_start_matches('-').to_string();
    // reverse
    x_str = x_str.chars().rev().collect::<String>();
    x_str = x_str.trim_start_matches('0').to_string();
    // back to integer
    if let Ok(res) = i32::from_str(&x_str) {
        res * if is_negative {
            -1
        } else {
            1
        }
    } else {
        0
    }
}

#[test]
fn test_reverse_integer() {
    let res = reverse(123);
    assert_eq!(res, 321);

    let res = reverse(-123);
    assert_eq!(res, -321);

    let res = reverse(120);
    assert_eq!(res, 21);
}