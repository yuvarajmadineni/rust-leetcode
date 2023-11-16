use crate::ListNode;

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut l = 0;

    let mut curr = &mut head;
    while curr.is_some() {
        curr = &mut curr.as_mut().unwrap().next;
        l += 1;
    }
    curr = &mut head;
    let remove_ind = l - n;
    let mut i = 1;

    if remove_ind == 0 {
        return head.as_mut().unwrap().next.take();
    }

    while i < remove_ind {
        curr = &mut curr.as_mut().unwrap().next;
        i += 1;
    }

    let next = &mut curr.as_mut().unwrap().next.take();
    std::mem::swap(&mut curr.as_mut().unwrap().next, &mut next.as_mut()?.next);

    next.as_mut().unwrap().next = None;
    head
}

// 1->2->3->4->5
// 2
// 5-2 = 3
