use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::{BinaryHeap, HashMap, HashSet},
    i32::MIN,
    mem::swap,
    rc::Rc,
};

use crate::car_fleet::car_fleet;
use crate::character_replacement::character_replacement;
use crate::check_inclusion::check_inclusion;
use crate::daily_temperatures::daily_temperatures;
use crate::eval_rpn::eval_rpn;
use crate::generate_paranthesis::generate_parenthesis;
use crate::is_valid_paranthesis::is_valid;
use crate::largest_rectangle_area::largest_rectangle_area;
use crate::max_sliding_window::max_sliding_window;
use crate::min_window::min_window;
use crate::minstack::Minstack;
use crate::search_matrix::search_matrix;

pub mod car_fleet;
pub mod character_replacement;
pub mod check_inclusion;
pub mod daily_temperatures;
pub mod eval_rpn;
pub mod generate_paranthesis;
pub mod is_valid_paranthesis;
pub mod largest_rectangle_area;
pub mod max_sliding_window;
pub mod min_window;
pub mod minstack;
pub mod search_matrix;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct AltListNode {
    pub val: i32,
    pub next: RefCell<Option<Rc<AltListNode>>>,
    // ↑ ↑ is more versatile than Option<Box<ListNode>>.
}

type Node = RefCell<Option<Rc<AltListNode>>>;
impl AltListNode {
    pub fn new(val: i32) -> Self {
        AltListNode {
            val,
            next: RefCell::new(None),
        }
    }

    // pub fn detect_cycle(head: Node) -> Node {
    //     // fn next(node_opt: &Node) -> Node {
    //     //     match node_opt {
    //     //         Some(node) => node.borrow().next.clone(),
    //     //         None => None,
    //     //     }
    //     // }
    //     let mut slow = head.clone();
    //     let mut fast = head.clone();
    //     let mut pos: Node = RefCell::new(None);

    //     loop {
    //         slow = slow.borrow_mut().unwrap().next.clone();
    //         fast = fast
    //             .borrow_mut()
    //             .unwrap()
    //             .next
    //             .borrow_mut()
    //             .unwrap()
    //             .next
    //             .clone();

    //         if fast.borrow_mut().is_none() {
    //             break;
    //         }

    //         if slow == fast {
    //             slow = head;
    //             while slow != fast {
    //                 slow = slow.borrow_mut().unwrap().next;
    //                 fast = fast.borrow_mut().unwrap().next.borrow_mut().unwrap().next;
    //             }
    //             pos = slow;
    //             break;
    //         }
    //     }
    //     pos
    // }
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut ptr: &mut Option<Box<ListNode>> = &mut head;

        while list1.is_some() && list2.is_some() {
            let l1 = &mut list1;
            let l2 = &mut list2;

            let tmp = if l1.as_mut().unwrap().val <= l2.as_mut().unwrap().val {
                l1
            } else {
                l2
            };
            swap(ptr, tmp);
            swap(tmp, &mut ptr.as_mut().unwrap().next);
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        if list1.is_none() {
            swap(ptr, &mut list2);
        } else {
            swap(ptr, &mut list1);
        }
        head

        //     match (list1, list2) {
        //         (Some(l1), None) => Some(l1),
        //         (None, Some(l2)) => Some(l2),
        //         (None, None) => None,
        //         (Some(l1), Some(l2)) => match l1.val <= l2.val {
        //             true => Some(Box::new(ListNode {
        //                 val: l1.val,
        //                 next: Self::merge_two_lists(l1.next, Some(l2)),
        //             })),
        //             false => Some(Box::new(ListNode {
        //                 val: l2.val,
        //                 next: Self::merge_two_lists(Some(l1), l2.next),
        //             })),
        //         },
        //     }
    }
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr_head = head;
        let mut prev = None;
        while let Some(mut node) = curr_head {
            curr_head = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
    }
}

fn main() {
    let v = vec![1, 3, 4, 2, 5, 1];
    let max_value = v.iter().max().unwrap();
    println!("Max value is {}", max_value);
    find_maximum(&v);

    let k = 3;
    let a = vec![1, 2, 3, 4, 5];
    let val = max_with_kele(a, k);
    println!("{}", val);

    let arr1 = vec![1, 3, 2, 4];
    let arr2 = vec![3, 1, 2, 4];
    let pre = prefix_common_array(arr1, arr2);
    let grid = vec![
        vec![0, 2, 1, 0],
        vec![4, 0, 0, 3],
        vec![1, 0, 0, 4],
        vec![0, 3, 2, 0],
    ];
    // find_max_fish(grid);
    let node3 = ListNode::new(3);
    let node2 = ListNode {
        val: 2,
        next: Some(Box::new(node3)),
    };
    let node1 = ListNode {
        val: 1,
        next: Some(Box::new(node2)),
    };
    let second_node3 = ListNode::new(6);
    let second_node2 = ListNode {
        val: 5,
        next: Some(Box::new(second_node3)),
    };
    let second_node1 = ListNode {
        val: 4,
        next: Some(Box::new(second_node2)),
    };

    let mut third_node3 = ListNode::new(2);
    // third_node2.next = Some(Box::new(third_node1.clone()));
    println!("node {:?}", node1);
    let res = running_sum(vec![1, 2, 3, 4]);
    println!("{:?}", res);
    let s = find_pivot_index(vec![1, 7, 3, 6, 5, 6]);
    println!("{}", s);
    // let is_valid = is_isomorphic("baba".to_string(), "bala".to_string());
    // println!("{}", is_valid);
    let is_sub = is_subsequence("axc".to_string(), "ahdbc".to_string());
    println!("{}", is_sub);
    let merged_list =
        ListNode::merge_two_lists(Some(Box::new(node1.clone())), Some(Box::new(second_node1)));
    println!("{:?}", merged_list);
    let reversed_list = ListNode::reverse_list(Some(Box::new(node1.clone())));
    println!("{:?}", reversed_list);
    let middle_node = ListNode::middle_node(Some(Box::new(node1.clone())));
    println!("{:?}", middle_node);
    let v = vec![2, 7, 11, 13];
    let sum = two_sum(v, 9);
    println!("{:?}", sum);
    let k = group_anagrams(vec!["abc".to_string(), "bca".to_string()]);
    println!("{:?}", k);
    let res = top_k_frequent(vec![1, 1, 2, 3, 2], 3);
    println!("{:?}", res);
    let f = top_k_freq(vec![1, 1, 1, 2, 2, 3], 2);
    println!("{:?}", f);
    let r = product_except_self(vec![1, 2, 3, 4]);
    println!("{:?}", r);
    let sudoku = is_valid_sudoku(vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]);
    println!("{}", sudoku);
    let result = longest_consecutive(vec![100, 1, 200, 3, 4, 2]);
    println!("{}", result);
    let pal = is_palindrome("race a car".to_string());
    println!("{}", pal);
    let sum = two_sum_constant_space(vec![2, 7, 11, 15], 9);
    println!("{:?}", sum);
    let tsum = three_sum(vec![-2, 0, 0, 2, 2]);
    println!("{:?}", tsum);
    let total_water = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("total sum: {}", total_water);
    let trapsum = trap(vec![4, 2, 0, 3, 2, 5]);
    println!("trapsum: {}", trapsum);
    let profit = max_profit([7, 6, 4, 3, 1].to_vec());
    println!("profit: {}", profit);
    let longest_sub = length_of_longest_substring("aabaab!bb".to_string());
    println!("longest_substring: {}", longest_sub);
    let replace = character_replacement("ABAB".to_string(), 2);
    println!("replace: {}", replace);
    let inclusion = check_inclusion("ab".to_string(), "eidbaooo".to_string());
    println!(" inclusion: {}", inclusion);
    let min_length_window = min_window("aaaaaaaaaaaabbbbbcdd".to_string(), "abcdd".to_string());
    println!("min_length_window: {}", min_length_window);
    let max_window = max_sliding_window([9, 10, 9, -7, -4, -8, 2, -6].to_vec(), 5);
    println!("max_window: {:?}", max_window);
    assert_eq!([10, 10, 9, 2].to_vec(), max_window);
    let vaid_paranthesis = is_valid("()".to_string());
    println!("vaid_paranthesis: {}", vaid_paranthesis);
    let mut minimum_stack = Minstack::new();
    minimum_stack.push(2);
    minimum_stack.push(1);
    minimum_stack.push(2);
    let min_ele = minimum_stack.get_min();
    println!("minimum_stack: {:?} {}", minimum_stack, min_ele);
    let eval_res = eval_rpn(["2".to_string()].to_vec());
    println!("eval_res: {:?}", eval_res);
    let paranthesis = generate_parenthesis(3);
    println!("{:?}", paranthesis);
    let min_temperatures = daily_temperatures([89, 62, 70, 58, 47, 47, 46, 76, 100, 70].to_vec());
    println!("{:?}", min_temperatures);
    let fleet = car_fleet(10, [6, 8].to_vec(), [3, 2].to_vec());
    println!("{}", fleet);
    let area = largest_rectangle_area([2, 1, 5, 6, 2, 3].to_vec());
    println!("max area of rectangle is {}", area);
    let search_2d_array = search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        3,
    );
    println!("search matrix {}", search_2d_array);
}

fn find_maximum(arr: &Vec<i32>) {
    let mut maximum = &MIN;

    for val in arr {
        if val > maximum {
            maximum = val;
        }
    }
    println!("max is {}", maximum);
}

fn max_with_kele(arr: Vec<i32>, k: i32) -> i32 {
    let max_value = arr.iter().max().unwrap();
    let sum_to_max = (max_value - 1) * max_value / 2;
    let curr_sum = (max_value + k - 1) * (max_value + k) / 2;

    curr_sum - sum_to_max
}

fn prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let i = a.len();
    let mut sum = 0;
    let mut out = vec![];
    for index in 0..i {
        insert_to_map(a[index], &mut map);

        sum = sum + add_values(map.get(&a[index]));
        insert_to_map(b[index], &mut map);
        sum = sum + add_values(map.get(&b[index]));
        out.push(sum);
    }

    out
}

fn insert_to_map(key: i32, m: &mut HashMap<i32, i32>) {
    m.entry(key).and_modify(|e| *e += 1).or_insert(0);
}

fn add_values(v: Option<&i32>) -> i32 {
    match v {
        Some(val) => *val,
        None => 0,
    }
}

// fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
//     for i in 0..grid.len() {
//         for j in 0..grid[i].len() {}
//     }
// }

fn running_sum(v: Vec<i32>) -> Vec<i32> {
    let mut out = vec![v[0]];

    for i in 1..v.len() {
        out.push(out.last().unwrap() + v[i]);
    }
    out
}

fn find_pivot_index(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut cur_sum = 0;
    let mut pivot: i32 = 0;
    for i in 0..nums.len() {
        cur_sum = cur_sum + nums[i];
        println!("cur{} total {}", cur_sum - nums[i], sum - cur_sum);
        if cur_sum - nums[i] == sum - cur_sum {
            pivot = i as i32;
            return pivot;
        }
    }
    pivot
}

fn is_isomorphic(s: String, t: String) -> bool {
    let mut m1 = [0; 256];
    let mut m2 = [0; 256];

    s.chars().zip(t.chars()).all(|(s, t)| {
        let check = m1[s as usize] == m2[t as usize];
        m1[s as usize] = s as usize;
        m2[t as usize] = s as usize;
        check
    })
}

fn is_subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        //can be optimised  we can convert into char array ahead as s.char() is being called lot ot
        //times here
        if s.chars().nth(i) == t.chars().nth(j) {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }
    return i == s.len();
}
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        if map.contains_key(&nums[i]) {
            return true;
        }
        map.insert(nums[i], true);
    }
    false
}
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, n) in nums.into_iter().enumerate() {
        let diff = target - n;

        if let Some(&j) = map.get(&diff) {
            return vec![i as i32, j as i32];
        }
        map.insert(n, i);
    }
    unreachable!()
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        map.entry(key).or_insert(vec![]).push(s);
    }

    map.into_values().collect()
}

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        let val = nums[i];
        map.entry(val).and_modify(|c| *c += 1).or_insert(0);
    }
    let mut freq: Vec<Vec<Option<i32>>> = vec![];
    for _i in 0..nums.len() + 1 {
        freq.push([].to_vec());
    }
    for (key, val) in map.into_iter() {
        freq[val].push(Some(key));
    }
    let mut res: Vec<i32> = vec![];

    let mut count = 0;
    for i in (0..freq.len()).rev() {
        for j in 0..freq[i].len() {
            if count == k {
                return res;
            }
            if let Some(value) = freq[i][j] {
                count += 1;
                res.push(value);
            }
        }
    }
    res
}

fn top_k_freq(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut heap = BinaryHeap::with_capacity(nums.len());
    for i in 0..nums.len() {
        let val = nums[i];
        map.entry(val).and_modify(|c| *c += 1).or_insert(1);
    }

    map.into_iter().for_each(|x| {
        heap.push((x.1, x.0));
    });
    let mut res = vec![];
    for _i in 0..k {
        if let Some(v) = heap.pop() {
            res.push(v.1);
        }
    }
    res
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // let mut left_prod = nums.clone();
    // let mut right_prod = nums.clone();
    // for i in 1..nums.len() {
    //     left_prod[i] = left_prod[i - 1] * nums[i]
    // }
    //
    // for i in (1..nums.len() - 1).rev() {
    //     right_prod[i] = right_prod[i + 1] * nums[i];
    // }
    // let mut res = vec![];
    // for i in 0..nums.len() {
    //     if i == 0 {
    //         res.push(right_prod[i + 1]);
    //         continue;
    //     }
    //     if i == nums.len() - 1 {
    //         res.push(left_prod[i - 1]);
    //         continue;
    //     }
    //     res.push(left_prod[i - 1] * right_prod[i + 1]);
    // }
    // res
    let mut res = vec![];
    for _i in 0..nums.len() {
        res.push(1);
    }
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut ls = 1;
    let mut rs = 1;
    while i < nums.len() && j >= 0 {
        res[i] = ls * res[i];
        res[j] = rs * res[j];
        ls = ls * nums[i];
        rs = rs * nums[j];
        i += 1;
        if j != 0 {
            j -= 1
        };
    }
    res
}

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut set = HashSet::new();
    let mut grid: HashMap<(usize, usize), HashSet<char>> = HashMap::new();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let value = board[i][j];
            if value.is_digit(10) && !set.insert(value) {
                return false;
            }
        }
        set.clear();
    }

    for j in 0..board.len() {
        for i in 0..board.len() {
            let value = board[i][j];
            if value.is_digit(10) && !set.insert(value) {
                return false;
            }
        }
        set.clear();
    }

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let key = (i / 3, j / 3);
            println!("{:?}", key);
            if let Some(values) = grid.get_mut(&key) {
                let value = board[i][j];
                if value.is_digit(10) && !values.insert(value) {
                    return false;
                }
            } else {
                let mut hash = HashSet::new();
                hash.insert(board[i][j]);
                grid.entry(key).or_insert(hash);
            }
        }
    }
    true
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    };
    let s: HashSet<i32> = HashSet::from_iter(nums.clone());
    let mut max_sequence_len = 1;
    for v in &s {
        if s.contains(&(v - 1)) {
            continue;
        }
        let mut start = *v;
        let mut seq = 1;
        while s.contains(&(start + 1)) {
            seq += 1;
            start += 1;
        }
        if seq > max_sequence_len {
            max_sequence_len = seq;
        }
    }
    max_sequence_len
}

fn is_palindrome(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let mut i = 0;
    let new_s = s.as_bytes();
    let mut j = new_s.len() - 1;
    while i < j {
        let left = new_s[i].to_ascii_lowercase();
        let right = new_s[j].to_ascii_lowercase();
        if !left.is_ascii_alphanumeric() && !right.is_ascii_alphanumeric() {
            i += 1;
            j -= 1;
            continue;
        }
        if !right.is_ascii_alphanumeric() {
            j -= 1;
            continue;
        }
        if !left.is_ascii_alphanumeric() {
            i += 1;
            continue;
        }
        if left != right {
            return false;
        }

        i += 1;
        j -= 1;
    }
    true
}

fn two_sum_constant_space(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;
    while i < j {
        let curr_sum = numbers[i] + numbers[j];
        if curr_sum == target {
            return [(i + 1) as i32, (j + 1) as i32].to_vec();
        } else if curr_sum > target {
            j -= 1;
        } else if curr_sum < target {
            i += 1;
        } else {
            i += 1;
            j -= 1;
        };
    }

    return [0, 0].to_vec();
}

fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut i = 0;
    nums.sort();
    let mut res: Vec<Vec<i32>> = vec![];
    let mut j = i + 1;
    let mut k: usize = nums.len() - 1;
    while i < nums.len() {
        let curr = nums[i];
        while j < k {
            let remaining = nums[j] + nums[k];
            if curr + remaining > 0 {
                k -= 1;
            }
            if curr + remaining < 0 {
                j += 1;
            }
            if curr + remaining == 0 {
                res.push([nums[i], nums[j], nums[k]].to_vec());
                j += 1;
                while j < k && nums[j] == nums[j - 1] {
                    j += 1;
                }
            }
        }

        while i < nums.len() && nums[i] == curr {
            i += 1;
        }
        j = i + 1;
        k = nums.len() - 1;
    }
    res
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;
    let mut area = 0;
    while i < j {
        area = area.max(min(height[i], height[j]) * (j - i) as i32);
        if height[i] < height[j] {
            i += 1
        } else if height[i] > height[j] {
            j -= 1;
        } else {
            i += 1;
        }
    }
    area
}

fn trap(height: Vec<i32>) -> i32 {
    // let mut left_height = vec![0; height.len()];
    // let mut right_height = vec![0; height.len()];
    // let mut maxl = 0;
    // let mut maxr = 0;
    // for i in 0..height.len() {
    //     left_height[i] = maxl;
    //     maxl = max(maxl, height[i]);
    // }

    // for i in (0..height.len()).rev() {
    //     right_height[i] = maxr;
    //     maxr = max(maxr, height[i]);
    // }

    // let mut res = 0;
    // for i in 0..height.len() {
    //     res = res + max(0, min(left_height[i], right_height[i]) - height[i]);
    // }

    let mut res = 0;
    let mut i = 0;
    let mut j = height.len() - 1;
    let (mut maxl, mut maxr) = (height[i], height[j]);
    while i < j {
        if maxl <= maxr {
            res += max(0, maxl - height[i]);
            i = i + 1;
        } else {
            res += max(0, maxr - height[j]);
            j = j - 1;
        }
        maxl = maxl.max(height[i]);
        maxr = maxr.max(height[j]);
    }
    res
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_price = 0;
    let mut min_price = prices[0];
    for i in 1..prices.len() {
        max_price = max_price.max(prices[i] - min_price);
        min_price = min_price.min(prices[i]);
    }
    max_price
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut map = HashMap::new();
    let (mut start_ind, mut max_len) = (0, 0);
    for (curr_idx, curr_val) in s.chars().enumerate() {
        if let Some(prev_val) = map.insert(curr_val, curr_idx) {
            start_ind = start_ind.max(prev_val + 1);
        }
        max_len = max_len.max(curr_idx - start_ind + 1);
    }
    max_len as i32
}
