use crate::ListNode;

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut middle = middle(head).as_mut().unwrap().next.take();

    let curr = &mut middle;
    let mut prev = None;

    while curr.is_some() {
        std::mem::swap(&mut curr.as_mut().unwrap().next, &mut prev);
        std::mem::swap(curr, &mut prev);
    }
    std::mem::swap(curr, &mut prev);

    let mut left = head;
    let right = curr;

    while left.is_some() && right.is_some() {
        let l_next = &mut left.as_mut().unwrap().next;
        std::mem::swap(l_next, right);
        std::mem::swap(&mut l_next.as_mut().unwrap().next, right);
        left = &mut l_next.as_mut().unwrap().next;
    }
}

fn middle(mut head: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
    let mut fast = &head.clone();

    while fast.is_some() {
        fast = &fast.as_ref().unwrap().next;
        if fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            head = &mut head.as_mut().unwrap().next;
        }
    }
    head
}
