use std::collections::HashMap;

// 1 <= orders.length <= 5 * 10^4
// orders[i].length == 3
// 1 <= customerNamei.length, foodItemi.length <= 20
// customerNamei 和 foodItemi 由大小写英文字母及空格字符 ' ' 组成。
// tableNumberi 是 1 到 500 范围内的整数。
pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut food_names = Vec::new();
    let mut table_pre:Vec<HashMap<String, i32>> = vec![HashMap::new();501];
    for order in orders.iter() {
        food_names.push(order[2].clone());
        let table_num:usize = order[1].parse().unwrap();
        let table_pre_row = table_pre.get_mut(table_num).unwrap();
        if let Some(o) = table_pre_row.get_mut(&*order[2]) {
            *o += 1;
        } else {
            table_pre_row.insert(order[2].clone(), 1);
        }
    }

    food_names.sort();
    food_names.dedup();
    let mut table :Vec<Vec<String>> = Vec::new();
    for (i,row) in table_pre.iter().enumerate() {
        if row.is_empty() {
            continue;
        }
        let mut table_row = vec![i.to_string()];
        for food_name in food_names.iter() {
            table_row.push(row.get(food_name).unwrap_or(&0).to_string());
        }
        table.push(table_row);
    }
    food_names.insert(0, "Table".to_string());
    table.insert(0, food_names);

    table
}

#[test]
fn display_table_t0() {
    // 输入：orders = [["David","3","Ceviche"],["Corina","10","Beef Burrito"],["David","3","Fried Chicken"],["Carla","5","Water"],["Carla","5","Ceviche"],["Rous","3","Ceviche"]]
    // 输出：[["Table","Beef Burrito","Ceviche","Fried Chicken","Water"],["3","0","2","1","0"],["5","0","1","0","1"],["10","1","0","0","0"]]
    // 解释：
    // 点菜展示表如下所示：
    // Table,Beef Burrito,Ceviche,Fried Chicken,Water
    // 3    ,0           ,2      ,1            ,0
    // 5    ,0           ,1      ,0            ,1
    // 10   ,1           ,0      ,0            ,0
    // 对于餐桌 3：David 点了 "Ceviche" 和 "Fried Chicken"，而 Rous 点了 "Ceviche"
    // 而餐桌 5：Carla 点了 "Water" 和 "Ceviche"
    // 餐桌 10：Corina 点了 "Beef Burrito"
    let orders:Vec<Vec<String>> = vec![
        vec!["David","3","Ceviche"].into_iter().map(String::from).collect(),
        vec!["Corina","10","Beef Burrito"].into_iter().map(String::from).collect(),
        vec!["David","3","Fried Chicken"].into_iter().map(String::from).collect(),
        vec!["Carla","5","Water"].into_iter().map(String::from).collect(),
        vec!["Carla","5","Ceviche"].into_iter().map(String::from).collect(),
        vec!["Rous","3","Ceviche"].into_iter().map(String::from).collect()];
    let table:Vec<Vec<String>> = vec![
        vec!["Table","Beef Burrito","Ceviche","Fried Chicken","Water"].into_iter().map(String::from).collect(),
        vec!["3","0","2","1","0"].into_iter().map(String::from).collect(),
        vec!["5","0","1","0","1"].into_iter().map(String::from).collect(),
        vec!["10","1","0","0","0"].into_iter().map(String::from).collect()];
    assert_eq!(table, display_table(orders));
}