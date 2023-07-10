use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut map = HashMap::new();
    let (mut i, mut j) = (0, 0);
    let mut res: i32 = 0;
    let mut insert_key = true;
    while j < s.len() {
        if insert_key {
            map.entry(s.chars().nth(j))
                .and_modify(|c| *c += 1)
                .or_insert(1);
        };
        let most_freq_count = *map.values().max().unwrap();
        let window_len: usize = j - i + 1;
        if window_len - most_freq_count <= k as usize {
            res = res.max(window_len as i32);
            j += 1;
            insert_key = true;
        } else {
            if let Some(val) = map.get_mut(&s.chars().nth(i)) {
                *val = *val - 1;
            }
            i += 1;
            insert_key = false;
        }
    }
    res
}
