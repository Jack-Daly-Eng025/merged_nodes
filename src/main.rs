
pub struct Solution;

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


impl Solution {
    pub fn merge_nodes( mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let h = head.as_deref_mut().unwrap().next.take();
        let mut t = &h;
        let mut a = ListNode::new(0);
        let mut a_t = &mut a;
        let mut s = 0;
        while let Some(node) = t {
            if node.val==0 {
                a_t.next = Some(Box::new(ListNode::new(s)));
                a_t = a_t.next.as_deref_mut().unwrap();
                s=0;
            }
            else {
                s+=node.val;
            }
            t = &t.as_deref().unwrap().next;
        }
        a.next.take()

        
    }
}



fn main() {
    let list = vec![0,1,0,3,0,2,2,0];
    let mut nodes = None;
    let mut current = &mut nodes;
    for x in list {
        *current = Some(Box::new(ListNode::new(x)));
        current = &mut current.as_mut().unwrap().next;
    }
    let _merged_nodes = Solution::merge_nodes(nodes);

    drop(_merged_nodes);
}
