#[warn(dead_code)]
// Definition for singly-linked list.
 #[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }

 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }

//输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
//输出：7 -> 0 -> 8
//原因：342 + 465 = 807
fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1_node= l1;
    let mut l2_node= l2;
    let mut is_carry = false;
    let mut res_sum = vec![];
    loop {
        if l1_node == None && l2_node == None {
            if is_carry {
                // carry last one
                res_sum.push(1);
            }
            break;
        }

        let mut node_sum;
        if l1_node == None {
            node_sum = l2_node.as_ref().unwrap().val;
            l2_node = l2_node.unwrap().next;
        } else if l2_node == None {
            node_sum = l1_node.as_ref().unwrap().val;
            l1_node = l1_node.unwrap().next;
        } else {
            node_sum = l1_node.as_ref().unwrap().val + l2_node.as_ref().unwrap().val;
            l1_node = l1_node.unwrap().next;
            l2_node = l2_node.unwrap().next;
        }

        if is_carry { node_sum = node_sum + 1;}
        // next carry or NOT
        is_carry = node_sum > 9;
        if is_carry {node_sum = node_sum - 10;}
        res_sum.push(node_sum);
    }

    let mut res_node= None;
    res_sum.reverse();
    for sum in res_sum {
        res_node = Some(Box::new(ListNode {
            next: res_node.take(),
            val: sum
        }))
    }

    return res_node;
}

#[test]
fn test_add_two_numbers() {
    //342
    let node = Some(Box::new(
        ListNode {next: Some(Box::new(
            ListNode {next: Some(Box::new(
                ListNode::new(
                    3)
            )), val: 4}
        )), val: 2}
    ));
    //465
    let node2 = Some(Box::new(
        ListNode {next: Some(Box::new(
            ListNode {next: Some(Box::new(
                ListNode::new(
                    4)
            )), val: 6}
        )), val: 5}
    ));
    //807
    let result = add_two_numbers(node, node2);
    result.map(|res|{
        assert_eq!(res.val, 7);
        assert_eq!(res.next.as_ref().unwrap().val, 0);
        assert_eq!(res.next.as_ref().unwrap().next.as_ref().unwrap().val, 8);
    });
}