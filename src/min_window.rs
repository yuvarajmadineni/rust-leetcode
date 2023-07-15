use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    let s: Vec<char> = s.chars().collect();
    let mut start_ind = 0;
    let mut right_ind = 0;
    let (mut have, mut res_len, mut res) = (0, usize::MAX, (-1, -1));

    let mut t_map = HashMap::new();
    let mut s_map = HashMap::new();

    for (_i, c) in t.chars().enumerate() {
        t_map.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    let need = t_map.keys().len();

    while right_ind < s.len() {
        let char = s[right_ind];
        s_map.entry(char).and_modify(|x| *x += 1).or_insert(1);
        if t_map.contains_key(&char) && *t_map.get(&char).unwrap() == *s_map.get(&char).unwrap() {
            have += 1;
        }
        right_ind += 1;

        while have == need {
            if right_ind - start_ind + 1 < res_len {
                res = (start_ind as i32, right_ind as i32);
            }
            res_len = res_len.min(right_ind - start_ind + 1);
            let s_char = s[start_ind];
            if let Some(v) = s_map.get_mut(&s_char) {
                *v -= 1;
            }
            if t_map.contains_key(&s_char)
                && !(*t_map.get(&s_char).unwrap() <= *s_map.get(&s_char).unwrap())
            {
                have -= 1;
            }
            start_ind += 1;
        }
    }
    if res.0 > -1 && res.1 > -1 {
        return s[(res.0 as usize)..(res.1 as usize)]
            .into_iter()
            .collect::<String>();
    }
    return "".to_string();
}
