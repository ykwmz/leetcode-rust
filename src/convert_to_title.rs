
// 给定一个正整数，返回它在 Excel 表中相对应的列名称。
//
// 例如，
//
//     1 -> A
//     2 -> B
//     3 -> C
//     ...
//     26 -> Z
//     27 -> AA
//     28 -> AB
//     ...
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/excel-sheet-column-title
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn convert_to_title(column_number: i32) -> String {
    // let titles = ['Z','A','B','C','D','E','F','G','H','I','J','K','L'
    // ,'M','N','O','P','Q','R','S','T','U','V','W','X','Y'];

    let mut title = String::new();
    let mut quotient = column_number; //商
    loop {
        //余数
        let remainder = (quotient % 26) as u8;
        if remainder == 0 {
            title.insert(0, 'Z');
            quotient = quotient / 26 - 1 //A~Z不存在0补位，所以当余数为0时，使用26即Z进行补位，故而此处需减去一份商
        } else {
            title.insert(0, char::from((remainder - 1) as u8 + 'A' as u8));
            quotient = quotient / 26
        };

        if quotient == 0 {
            break;
        }
    }

    title
}

#[test]
fn convert_to_title_t3(){
    assert_eq!(String::from("AZ"), convert_to_title(52))
}

#[test]
fn convert_to_title_t2(){
    assert_eq!(String::from("ZY"), convert_to_title(701))
}

#[test]
fn convert_to_title_t1(){
    assert_eq!(String::from("AB"), convert_to_title(28))
}

#[test]
fn convert_to_title_t0(){
    assert_eq!(String::from("A"), convert_to_title(1))
}