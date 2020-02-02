fn zigzag_convert(s: String, num_rows: i32) -> String {
    if num_rows < 2 {
        return s;
    }
    let mut sub_chars = vec![String::new();num_rows as usize];
    let mut i = 0i32;
    let mut step = -1;
    for c in s.chars() {
        sub_chars[i as usize].push(c);
        if i == 0 || i == (num_rows - 1) {
            step = - step;
        }

        i = i + step;
    }

    let mut res = String::new();
    for sub_char in sub_chars {
        res.push_str(&sub_char);
    }
    return res;
}

#[test]
fn test_zigzag_convert() {
    let res = zigzag_convert(String::from("LEETCODEISHIRING"), 3);
    assert_eq!(res, "LCIRETOESIIGEDHN".to_string());

    let res = zigzag_convert(String::from("LEETCODEISHIRING"), 4);
    assert_eq!(res, "LDREOEIIECIHNTSG".to_string());

    let res = zigzag_convert(String::from("AB"), 1);
    assert_eq!(res, "AB".to_string());

    let res = zigzag_convert(String::from("AB"), 2);
    assert_eq!(res, "AB".to_string());
}