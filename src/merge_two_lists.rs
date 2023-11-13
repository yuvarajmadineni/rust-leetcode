use crate::linked_list::ListNode;
pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut r = &mut list2;
    while list1.is_some() {
        if r.is_none() || list1.as_ref().unwrap().val < r.as_ref().unwrap().val {
            std::mem::swap(&mut list1, r);
        }
        r = &mut r.as_mut()?.next;
    }
    list2
}
